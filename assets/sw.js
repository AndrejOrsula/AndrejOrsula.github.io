var cacheName = "portfolio_andrejorsula";
var filesToCache = [
    "./",
    "./index.html",
    "./portfolio_andrejorsula.js",
    "./portfolio_andrejorsula_bg.wasm",
];

self.addEventListener("install", function (e) {
    e.waitUntil(
        caches.open(cacheName).then(function (cache) {
            return cache.addAll(filesToCache);
        })
    );
});

self.addEventListener("fetch", function (e) {
    e.respondWith(
        caches.match(e.request).then(function (response) {
            return response || fetch(e.request);
        })
    );
});
