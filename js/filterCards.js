// Intentional full override of the pinned theme static/js/filterCards.js at
// revision 3896c1597c847ae31528d2e59b24e01d26a6485a. Drives the tag filter of
// templates/partials/filter_card_tags.html for any item list: the filtered
// items come from the list's data-filter-items selector, the links become
// toggles (role and aria-pressed are added here, so without JavaScript they stay
// plain links to the tag pages), Space and Enter activate them, and
// a polite live region announces how many items are visible after a change.
// Avoid template literals: the release asset graph treats them as dynamic
// loader sites (scripts/pipeline/asset_graph.py).
document.addEventListener('DOMContentLoaded', () => {
    const filterContainer = document.querySelector('.filter-controls');
    if (!filterContainer) return;
    const cards = document.querySelectorAll(filterContainer.dataset.filterItems || '.card');
    const filterLinks = filterContainer.querySelectorAll('a[data-filter]');
    const allProjectsFilter = document.querySelector('#all-projects-filter');
    const status = filterContainer.parentElement.querySelector('.filter-status');
    const noun = filterContainer.dataset.filterNoun || 'items';
    if (!cards.length || !filterLinks.length) return;
    if (allProjectsFilter) allProjectsFilter.style.display = 'block';
    filterLinks.forEach(link => {
        link.setAttribute('role', 'button');
        link.setAttribute('aria-pressed', link.classList.contains('active') ? 'true' : 'false');
    });

    // Create a Map for O(1) lookups of links by filter value.
    const linkMap = new Map(
        Array.from(filterLinks).map(link => [link.dataset.filter, link])
    );

    // Pre-process cards data for faster filtering.
    const cardData = Array.from(cards).map(card => ({
        element: card,
        tags: card.dataset.tags?.toLowerCase().split(',').filter(Boolean) ?? []
    }));

    function getTagSlugFromUrl(url) {
        return url.split('/').filter(Boolean).pop();
    }

    function getFilterFromHash() {
        if (!window.location.hash) return 'all';
        const hash = decodeURIComponent(window.location.hash.slice(1));
        const matchingLink = Array.from(filterLinks).find(link =>
            getTagSlugFromUrl(link.getAttribute('href')) === hash
        );
        return matchingLink?.dataset.filter ?? 'all';
    }

    function announce(filterValue, visible) {
        if (!status) return;
        const label = filterValue === 'all' ? 'all' : '“' + (linkMap.get(filterValue)?.textContent.trim() ?? filterValue) + '”';
        status.textContent = 'Showing ' + visible + ' of ' + cardData.length + ' ' + noun + ' (' + label + ').';
    }

    function setActiveFilter(filterValue, updateHash = true) {
        if (updateHash) {
            if (filterValue === 'all') {
                history.pushState(null, '', window.location.pathname);
            } else {
                const activeLink = linkMap.get(filterValue);
                if (activeLink) {
                    const tagSlug = getTagSlugFromUrl(activeLink.getAttribute('href'));
                    history.pushState(null, '', '#' + tagSlug);
                }
            }
        }
        requestAnimationFrame(() => {
            filterLinks.forEach(link => {
                const isActive = link.dataset.filter === filterValue;
                link.classList.toggle('active', isActive);
                link.setAttribute('aria-pressed', isActive ? 'true' : 'false');
            });
            let visible = 0;
            cardData.forEach(({ element, tags }) => {
                const shouldShow = filterValue === 'all' || tags.includes(filterValue);
                element.style.display = shouldShow ? '' : 'none';
                element.setAttribute('aria-hidden', shouldShow ? 'false' : 'true');
                if (shouldShow) visible += 1;
            });
            announce(filterValue, visible);
        });
    }

    filterContainer.addEventListener('click', e => {
        const link = e.target.closest('a[data-filter]');
        if (!link) return;
        e.preventDefault();
        setActiveFilter(link.dataset.filter);
    });

    // Links only activate on Enter natively; buttons also activate on Space.
    filterContainer.addEventListener('keydown', e => {
        const link = e.target.closest('a[data-filter]');
        if (!link) return;
        if (e.key === ' ' || e.key === 'Enter') {
            e.preventDefault();
            link.click();
        }
    });

    window.addEventListener('popstate', () => {
        setActiveFilter(getFilterFromHash(), false);
    });

    const initialFilter = getFilterFromHash();
    if (initialFilter !== 'all') {
        setActiveFilter(initialFilter, false);
    }
});
