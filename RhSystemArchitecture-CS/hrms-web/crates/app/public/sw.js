// Service worker for offline PWA support.
// Real caching strategy (app-shell precache + runtime API cache) to be filled in.

const CACHE = "hrms-shell-v1";

self.addEventListener("install", (event) => {
  // TODO: precache the app shell (index.html, wasm bundle, static assets).
  self.skipWaiting();
});

self.addEventListener("activate", (event) => {
  // TODO: clean up stale caches.
  self.clients.claim();
});

self.addEventListener("fetch", (event) => {
  // TODO: cache-first for the app shell, network-first for API calls.
});
