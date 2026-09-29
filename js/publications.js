(function () {
    "use strict";

    var resetDelay = 1500;

    function relatedElement(button, selector) {
        var entry = button.closest(".publication-entry");
        return entry ? entry.querySelector(selector) : null;
    }

    // The status is a live region that stays rendered (visually hidden when
    // idle) so screen readers announce text changes; never toggle `hidden`.
    function setStatus(button, text, visible) {
        var status = relatedElement(button, ".publication-copy-status");
        if (!status) return;
        status.textContent = text;
        status.classList.toggle("visually-hidden", !visible);
    }

    function showFallback(button) {
        var fallback = relatedElement(button, ".publication-citation-fallback");
        if (!fallback) return;
        fallback.hidden = false;
        fallback.focus();
        fallback.select();
    }

    function clearResetTimer(button) {
        if (button.publicationResetTimer) {
            window.clearTimeout(button.publicationResetTimer);
            button.publicationResetTimer = null;
        }
    }

    function restoreButton(button) {
        var label = button.querySelector("span");
        var icon = button.querySelector("svg");
        if (label) label.textContent = "BibTeX";
        if (icon) icon.style.display = "";
        button.classList.remove("copied");
    }

    function copyCitation(button) {
        if (button.dataset.copyState === "pending") return;

        var citation = button.getAttribute("data-bibtex");
        if (!citation) return;

        var label = button.querySelector("span");
        var icon = button.querySelector("svg");
        var clipboard = window.navigator && window.navigator.clipboard;
        var writeText = clipboard && typeof clipboard.writeText === "function"
            ? clipboard.writeText.bind(clipboard)
            : null;

        clearResetTimer(button);
        restoreButton(button);
        button.dataset.copyState = "pending";
        // aria-disabled (not disabled) keeps keyboard focus on the button; the
        // copyState guard above ignores activations while pending.
        button.setAttribute("aria-disabled", "true");

        var write = writeText
            ? Promise.resolve().then(function () { return writeText(citation); })
            : Promise.reject(new Error("Clipboard API unavailable"));

        write.then(function () {
            if (label) label.textContent = "Copied!";
            if (icon) icon.style.display = "none";
            button.classList.add("copied");
            setStatus(button, "Citation copied to clipboard.", true);
            button.publicationResetTimer = window.setTimeout(function () {
                restoreButton(button);
                setStatus(button, "", false);
            }, resetDelay);
        }, function () {
            restoreButton(button);
            setStatus(button, "Copy unavailable. Select the citation below.", true);
            showFallback(button);
        }).then(function () {
            button.dataset.copyState = "idle";
            button.removeAttribute("aria-disabled");
        });
    }

    if (document.documentElement.dataset.publicationsClipboardBound === "true") return;
    document.documentElement.dataset.publicationsClipboardBound = "true";
    document.addEventListener("click", function (event) {
        var target = event.target;
        var button = target && target.closest ? target.closest(".publication-copy-bib") : null;
        if (button) copyCitation(button);
    });
}());
