// Mobile navigation for templates/partials/nav.html. Loaded as an external
// deferred script so the Content-Security-Policy needs no 'unsafe-inline'.
(function () {
    "use strict";

    var btn = document.querySelector(".nav-hamburger");
    var nav = document.querySelector(".navbar");
    if (!btn || !nav) return;

    function isOpen() {
        return nav.classList.contains("nav-open");
    }

    function setOpen(open, restoreFocus) {
        nav.classList.toggle("nav-open", open);
        btn.setAttribute("aria-expanded", open ? "true" : "false");
        if (!open && restoreFocus) btn.focus();
    }

    // The search modal is opened from inside the menu; interacting with it
    // must not close the menu that holds the control focus returns to.
    function inSearchModal(node) {
        return Boolean(node && node.closest && node.closest("#searchModal"));
    }

    // Toggle hamburger menu
    btn.addEventListener("click", function () {
        setOpen(!isOpen(), false);
    });

    // Close on nav link click
    nav.querySelectorAll(".nav-navs .nav-links").forEach(function (a) {
        a.addEventListener("click", function () {
            setOpen(false, false);
        });
    });

    // Close on outside click
    document.addEventListener("click", function (e) {
        if (isOpen() && !nav.contains(e.target) && !inSearchModal(e.target)) setOpen(false, false);
    });

    // Close when keyboard focus leaves the menu (e.g. Tab past its last item)
    nav.addEventListener("focusout", function (e) {
        var next = e.relatedTarget;
        if (isOpen() && next && !nav.contains(next) && !inSearchModal(next)) setOpen(false, false);
    });

    // Close on Escape and return focus to the toggle. Escape inside the search
    // modal only closes the modal.
    document.addEventListener("keydown", function (e) {
        if (e.key === "Escape" && isOpen() && !inSearchModal(e.target)) setOpen(false, true);
    });

    // Move search + dark/light theme toggle into the hamburger dropdown on mobile
    var sb = nav.querySelector(".search-icon");
    var ts = nav.querySelector(".theme-switcher");
    var dd = nav.querySelector(".nav-navs");
    var ac = nav.querySelector(".nav-actions");
    if (dd && ac) {
        var row = document.createElement("div");
        row.className = "nav-tools-row";
        var mq = window.matchMedia("(max-width: 960px)");
        var syncTools = function (e) {
            if (e.matches) {
                if (sb) row.appendChild(sb);
                if (ts) row.appendChild(ts);
                dd.appendChild(row);
            } else {
                if (sb) ac.appendChild(sb);
                if (ts) ac.appendChild(ts);
                if (row.parentNode) row.parentNode.removeChild(row);
                setOpen(false, false);
            }
        };
        mq.addEventListener("change", syncTools);
        syncTools(mq);
    }
}());
