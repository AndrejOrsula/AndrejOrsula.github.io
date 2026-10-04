(function () {
    "use strict";

    if (document.documentElement.dataset.wasmDemoScriptBound === "true") return;
    document.documentElement.dataset.wasmDemoScriptBound = "true";

    var STARTUP_TIMEOUT_MS = 20000;
    var containers = Array.prototype.slice.call(document.querySelectorAll("[data-wasm-demo]"));
    var records = new WeakMap();
    var focusRevision = 0;

    document.addEventListener("focusin", function () { focusRevision += 1; }, true);

    function getRecord(container) {
        var record = records.get(container);
        if (!record) {
            record = { generation: 0, timer: null, abort: null, frame: null, navigationStarted: false,
                focusIntent: false, focusRevision: 0 };
            records.set(container, record);
        }
        return record;
    }

    function setStatus(container, message, visible, isError) {
        var status = container.querySelector(".wasm-status");
        if (!status) return;
        status.textContent = message;
        status.hidden = !visible;
        status.setAttribute("aria-live", isError ? "assertive" : "polite");
        status.setAttribute("role", isError ? "alert" : "status");
    }

    // `mode` is "retry", "idle" or empty (visibility only). The button is named
    // by its visible text; a project overlay's aria-label only appends the demo
    // title after that text, so the visible label stays part of the name.
    function setOverlay(container, visible, mode) {
        var overlay = container.querySelector(".wasm-overlay");
        if (!overlay) return;
        overlay.hidden = !visible;
        overlay.style.display = visible ? "" : "none";
        if (!mode) return;
        var isProject = container.dataset.demoKind === "project";
        var text = mode === "retry" ? "Try again" : (isProject ? "Open preview" : "Load demo");
        var buttonLabel = overlay.querySelector(".wasm-btn");
        if (buttonLabel) buttonLabel.textContent = text;
        if (overlay.hasAttribute("aria-label")) {
            var frame = container.querySelector(".wasm-iframe");
            var demoTitle = frame ? frame.getAttribute("title") : "";
            var action = mode === "retry" ? "Try again: open preview" : "Open preview";
            overlay.setAttribute("aria-label", demoTitle ? action + " for " + demoTitle : action);
        }
    }

    function cancelPending(container) {
        var record = getRecord(container);
        record.generation += 1;
        if (record.timer !== null) window.clearTimeout(record.timer);
        if (record.abort) record.abort.abort();
        record.timer = null;
        record.abort = null;
        return record;
    }

    function setFrameVisibility(container, visible) {
        var frame = container.querySelector(".wasm-iframe");
        if (frame) frame.hidden = !visible;
        var poster = container.querySelector(".wasm-poster");
        if (poster) poster.hidden = visible;
    }

    function showError(container, message, generation) {
        var record = getRecord(container);
        if (generation !== undefined && generation !== record.generation) return;
        var wasLoading = container.dataset.wasmState === "loading";
        var detail = message || "The interactive demo could not start. Try again.";
        cancelPending(container);
        replaceFrame(container);
        container.dataset.wasmState = "error";
        container.removeAttribute("aria-busy");
        container.classList.remove("wasm-loaded");
        container.classList.add("wasm-error");
        setFrameVisibility(container, false);
        setStatus(container, detail, true, true);
        setOverlay(container, true, "retry");
        var unload = container.querySelector(".wasm-unload");
        if (unload) unload.hidden = true;
        var overlay = container.querySelector(".wasm-overlay");
        if (overlay && (!wasLoading || (record.focusIntent && record.focusRevision === focusRevision))) overlay.focus();
        record.focusIntent = false;
    }

    function markLoaded(container, generation) {
        var record = getRecord(container);
        if (generation !== undefined && generation !== record.generation) return;
        if (container.dataset.wasmState !== "loading") return;
        container.dataset.wasmState = "loaded";
        container.removeAttribute("aria-busy");
        container.classList.remove("wasm-error");
        container.classList.add("wasm-loaded");
        setFrameVisibility(container, true);
        setStatus(container, "", false, false);
        setOverlay(container, false, "");
        var unload = container.querySelector(".wasm-unload");
        if (unload) unload.hidden = false;
        if (record.timer !== null) window.clearTimeout(record.timer);
        record.timer = null;
        if (record.abort) record.abort.abort();
        record.abort = null;
        if (record.focusIntent && record.focusRevision === focusRevision) {
            var frame = container.querySelector(".wasm-iframe");
            if (frame) frame.focus();
        }
        record.focusIntent = false;
    }

    function replaceFrame(container) {
        var oldFrame = container.querySelector(".wasm-iframe");
        if (!oldFrame) return null;
        var replacement = oldFrame.cloneNode(false);
        replacement.removeAttribute("srcdoc");
        replacement.setAttribute("src", "about:blank");
        oldFrame.replaceWith(replacement);
        var record = getRecord(container);
        record.frame = replacement;
        record.navigationStarted = false;
        bindFrame(container, replacement);
        return replacement;
    }

    function failHttp(container, generation, response) {
        if (response.status >= 400 && response.status !== 405 && response.status !== 501) {
            showError(container, "The interactive demo could not load (HTTP " + response.status + "). Try again.", generation);
            return true;
        }
        return false;
    }

    // An embedded document sees the system color scheme, not the page's theme
    // toggle, so a calculator demo gets the page theme as `#light` or `#dark`
    // and follows changes to it (demos/calc/src/lib.rs).
    function pageTheme() {
        var theme = document.documentElement.getAttribute("data-theme");
        if (theme === "light" || theme === "dark") return theme;
        return window.matchMedia && window.matchMedia("(prefers-color-scheme: dark)").matches ? "dark" : "light";
    }

    function themedSource(container, source) {
        return container.dataset.demoKind === "project" ? source : source.split("#")[0] + "#" + pageTheme();
    }

    function syncThemes() {
        containers.forEach(function (container) {
            if (container.dataset.demoKind === "project") return;
            var frame = container.querySelector(".wasm-iframe");
            var current;
            try { current = frame && frame.contentWindow.location.href; } catch (_) { return; }
            if (!current || current === "about:blank") return;
            var themed = themedSource(container, current);
            // A fragment-only navigation: the demo keeps running and no
            // history entry is added.
            if (themed !== current) frame.contentWindow.location.replace(themed);
        });
    }

    function setFrameSource(record, frame, source) {
        record.navigationStarted = true;
        frame.setAttribute("src", source);
    }

    function navigate(container, source, generation) {
        var record = getRecord(container);
        var frame = container.querySelector(".wasm-iframe");
        if (!frame || generation !== record.generation) return;
        if (!window.fetch) {
            setFrameSource(record, frame, themedSource(container, source));
            return;
        }

        var abort = typeof window.AbortController === "function" ? new window.AbortController() : null;
        record.abort = abort;
        var options = { method: "HEAD", credentials: "same-origin", cache: "no-store" };
        if (abort) options.signal = abort.signal;
        window.fetch(source, options).then(function (response) {
            if (generation !== record.generation || container.dataset.wasmState !== "loading") return;
            if (!failHttp(container, generation, response)) setFrameSource(record, frame, themedSource(container, source));
        }, function (error) {
            if (generation !== record.generation || container.dataset.wasmState !== "loading" ||
                (error && error.name === "AbortError")) return;
            setFrameSource(record, frame, themedSource(container, source));
        });
    }

    function activate(container, keyboardInitiated) {
        if (container.dataset.wasmState === "loading" || container.dataset.wasmState === "loaded") return;
        var frame = container.querySelector(".wasm-iframe");
        if (!frame) return;
        var source = frame.getAttribute("data-src");
        if (!source) {
            showError(container, "This demo has no source URL.");
            return;
        }
        var record = cancelPending(container);
        frame = replaceFrame(container);
        if (!frame) return;
        record.frame = frame;
        var generation = record.generation;
        record.focusIntent = keyboardInitiated === true;
        record.focusRevision = focusRevision;
        container.dataset.wasmState = "loading";
        container.setAttribute("aria-busy", "true");
        container.classList.remove("wasm-error");
        container.classList.add("wasm-loaded");
        setFrameVisibility(container, true);
        setStatus(container, "Loading interactive demo…", true, false);
        setOverlay(container, false, "");
        var unload = container.querySelector(".wasm-unload");
        if (unload) unload.hidden = false;

        if (container.dataset.demoKind === "project") {
            record.timer = window.setTimeout(function () {
                showError(container, "The demo took too long to start. Try again, or open the full-page demo.", generation);
            }, STARTUP_TIMEOUT_MS);
        }
        navigate(container, source, generation);
    }

    function unload(container) {
        var record = cancelPending(container);
        record.focusIntent = false;
        var wasActive = container.dataset.wasmState === "loading" || container.dataset.wasmState === "loaded";
        var frame = replaceFrame(container);
        if (!frame) return;
        record.frame = frame;
        container.dataset.wasmState = "idle";
        container.removeAttribute("aria-busy");
        container.classList.remove("wasm-loaded", "wasm-error");
        setFrameVisibility(container, false);
        setOverlay(container, true, "idle");
        setStatus(container, wasActive ? "Preview closed." : "", wasActive, false);
        var overlay = container.querySelector(".wasm-overlay");
        if (overlay) overlay.focus();
        var unloadButton = container.querySelector(".wasm-unload");
        if (unloadButton) unloadButton.hidden = true;
    }

    function bindFrame(container, frame) {
        frame.addEventListener("load", function () {
            if (frame !== container.querySelector(".wasm-iframe")) return;
            var record = getRecord(container);
            if (container.dataset.wasmState !== "loading") return;
            if (!record.navigationStarted || !isRequestedDocumentLoaded(frame)) return;
            if (container.dataset.demoKind !== "project") {
                markLoaded(container, record.generation);
            } else {
                setStatus(container, "Demo page loaded. Waiting for it to start…", true, false);
            }
        });
        frame.addEventListener("error", function () {
            if (frame === container.querySelector(".wasm-iframe")) showError(container, "The interactive demo could not load. Try again.");
        });
    }

    function isRequestedDocumentLoaded(frame) {
        try {
            var current = frame.contentDocument;
            if (!current || current.readyState !== "complete") return false;
            var currentUrl = new URL(current.URL || current.location.href, window.location.href);
            var requestedUrl = new URL(frame.getAttribute("data-src"), window.location.href);
            currentUrl.hash = "";
            requestedUrl.hash = "";
            return currentUrl.href === requestedUrl.href;
        } catch (_) {
            return false;
        }
    }

    function bindContainer(container) {
        if (container.dataset.wasmBound === "true") return;
        container.dataset.wasmBound = "true";
        var frame = container.querySelector(".wasm-iframe");
        var overlay = container.querySelector(".wasm-overlay");
        if (!frame) return;
        var record = getRecord(container);
        record.frame = frame;
        bindFrame(container, frame);

        if (overlay) {
            overlay.addEventListener("click", function (event) {
                activate(container, event.detail === 0 && document.activeElement === overlay);
            });
            overlay.addEventListener("keydown", function (event) {
                if (event.key === "Enter" || event.key === " ") {
                    event.preventDefault();
                    activate(container, true);
                }
            });
        }
        var unloadButton = container.querySelector(".wasm-unload");
        if (unloadButton) unloadButton.addEventListener("click", function () { unload(container); });

        if (frame.getAttribute("src") && frame.getAttribute("src") !== "about:blank") {
            record.navigationStarted = true;
            container.dataset.wasmState = "loading";
            container.classList.add("wasm-loaded");
            if (container.dataset.demoKind === "project") {
                container.setAttribute("aria-busy", "true");
                record.timer = window.setTimeout(function () {
                    showError(container, "The demo took too long to start. Try again, or open the full-page demo.", record.generation);
                }, STARTUP_TIMEOUT_MS);
            }
        } else {
            container.dataset.wasmState = "idle";
            setFrameVisibility(container, false);
        }
    }

    containers.forEach(bindContainer);

    if (window.MutationObserver) {
        new window.MutationObserver(syncThemes).observe(document.documentElement, { attributes: true, attributeFilter: ["data-theme"] });
    }
    if (window.matchMedia) {
        var systemScheme = window.matchMedia("(prefers-color-scheme: dark)");
        if (systemScheme.addEventListener) systemScheme.addEventListener("change", syncThemes);
    }

    window.addEventListener("message", function (event) {
        var data = event.data;
        if (!data || typeof data !== "object" || Array.isArray(data)) return;
        containers.forEach(function (container) {
            var frame = container.querySelector(".wasm-iframe");
            if (!frame || event.source !== frame.contentWindow) return;
            var source = frame.getAttribute("data-src");
            var expectedOrigin;
            try { expectedOrigin = new URL(source, window.location.href).origin; } catch (_) { return; }
            if (event.origin !== expectedOrigin) return;
            var id = container.getAttribute("data-demo-id");
            var isProject = container.dataset.demoKind === "project";

            if (isProject) {
                if (data.protocol !== 1 || typeof data.id !== "string" || data.id !== id) return;
                if (data.type === "wasm-demo-ready") {
                    if (typeof data.build !== "string" || data.build.trim().length === 0) return;
                    markLoaded(container, getRecord(container).generation);
                } else if (data.type === "wasm-demo-error") {
                    var includesBuild = Object.prototype.hasOwnProperty.call(data, "build");
                    var hasBuild = typeof data.build === "string" && data.build.trim().length > 0;
                    if (includesBuild && !hasBuild) return;
                    if (!hasBuild && (data.phase !== "startup" || container.dataset.wasmState !== "loading")) return;
                    var message = typeof data.message === "string" && data.message.trim() ? data.message :
                        "The interactive demo stopped unexpectedly.";
                    showError(container, message, getRecord(container).generation);
                }
                return;
            }

            // Preserve the calculator's historical runtime-error message shape.
            if ((data.type === "wasm-demo-error" || data.type === "wasm-error") && (!data.id || data.id === id)) {
                showError(container, data.message || data.error || "The interactive demo stopped unexpectedly.");
            } else if (data.type === "wasm-demo-ready" && (!data.id || data.id === id)) {
                markLoaded(container, getRecord(container).generation);
            }
        });
    });
}());
