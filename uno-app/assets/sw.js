/**
 * UNO App Service Worker
 *
 * Provides offline support with intelligent caching strategies:
 * - Cache-first for static assets (CSS, JS, images)
 * - Network-first for API calls with offline fallback
 * - Stale-while-revalidate for HTML pages
 *
 * @version 1.0.0
 */

const CACHE_VERSION = 'uno-v1';
const STATIC_CACHE = `${CACHE_VERSION}-static`;
const DYNAMIC_CACHE = `${CACHE_VERSION}-dynamic`;
const API_CACHE = `${CACHE_VERSION}-api`;

// Assets to precache on install
const PRECACHE_ASSETS = [
  '/',
  '/pkg/uno-app.css',
  '/pkg/uno-app.js',
  '/pkg/uno-app_bg.wasm',
  '/offline.html',
];

// API endpoints to cache
const CACHEABLE_API_ROUTES = [
  '/api/v1/licenses/variants',
  '/api/v1/stats',
  '/api/v1/faq',
];

// Cache durations (in seconds)
const CACHE_DURATIONS = {
  static: 7 * 24 * 60 * 60,  // 7 days
  api: 5 * 60,                // 5 minutes
  dynamic: 24 * 60 * 60,      // 1 day
};

/**
 * Install event - precache static assets
 */
self.addEventListener('install', (event) => {
  console.log('[SW] Installing service worker...');

  event.waitUntil(
    caches.open(STATIC_CACHE)
      .then((cache) => {
        console.log('[SW] Precaching static assets');
        return cache.addAll(PRECACHE_ASSETS.map(url => {
          return new Request(url, { cache: 'reload' });
        })).catch(err => {
          console.warn('[SW] Precache failed for some assets:', err);
        });
      })
      .then(() => self.skipWaiting())
  );
});

/**
 * Activate event - clean up old caches
 */
self.addEventListener('activate', (event) => {
  console.log('[SW] Activating service worker...');

  event.waitUntil(
    caches.keys()
      .then((cacheNames) => {
        return Promise.all(
          cacheNames
            .filter((name) => name.startsWith('uno-') && !name.startsWith(CACHE_VERSION))
            .map((name) => {
              console.log('[SW] Deleting old cache:', name);
              return caches.delete(name);
            })
        );
      })
      .then(() => self.clients.claim())
  );
});

/**
 * Fetch event - handle requests with appropriate caching strategy
 */
self.addEventListener('fetch', (event) => {
  const { request } = event;
  const url = new URL(request.url);

  // Skip non-GET requests
  if (request.method !== 'GET') {
    return;
  }

  // Skip chrome-extension and other non-http requests
  if (!url.protocol.startsWith('http')) {
    return;
  }

  // Handle different request types
  if (isApiRequest(url)) {
    event.respondWith(networkFirstWithCache(request, API_CACHE));
  } else if (isStaticAsset(url)) {
    event.respondWith(cacheFirstWithNetwork(request, STATIC_CACHE));
  } else if (isNavigationRequest(request)) {
    event.respondWith(staleWhileRevalidate(request, DYNAMIC_CACHE));
  } else {
    event.respondWith(networkFirstWithCache(request, DYNAMIC_CACHE));
  }
});

/**
 * Check if request is for API
 */
function isApiRequest(url) {
  return url.pathname.startsWith('/api/');
}

/**
 * Check if request is for static asset
 */
function isStaticAsset(url) {
  const staticExtensions = ['.js', '.css', '.wasm', '.png', '.jpg', '.jpeg', '.gif', '.svg', '.woff', '.woff2', '.ttf'];
  return staticExtensions.some(ext => url.pathname.endsWith(ext)) || url.pathname.startsWith('/pkg/');
}

/**
 * Check if request is a navigation request
 */
function isNavigationRequest(request) {
  return request.mode === 'navigate' ||
         (request.method === 'GET' && request.headers.get('accept')?.includes('text/html'));
}

/**
 * Cache-first strategy: Try cache, fallback to network
 * Best for: Static assets that rarely change
 */
async function cacheFirstWithNetwork(request, cacheName) {
  const cache = await caches.open(cacheName);
  const cachedResponse = await cache.match(request);

  if (cachedResponse) {
    // Return cached response, but fetch in background for freshness
    fetchAndCache(request, cacheName);
    return cachedResponse;
  }

  return fetchAndCache(request, cacheName);
}

/**
 * Network-first strategy: Try network, fallback to cache
 * Best for: API calls and dynamic content
 */
async function networkFirstWithCache(request, cacheName) {
  const cache = await caches.open(cacheName);

  try {
    const response = await fetch(request);

    // Only cache successful responses
    if (response.ok) {
      const responseClone = response.clone();
      cache.put(request, responseClone);
    }

    return response;
  } catch (error) {
    console.log('[SW] Network failed, trying cache:', request.url);
    const cachedResponse = await cache.match(request);

    if (cachedResponse) {
      return cachedResponse;
    }

    // Return offline fallback for navigation requests
    if (isNavigationRequest(request)) {
      return caches.match('/offline.html');
    }

    // Return error response for API
    if (isApiRequest(new URL(request.url))) {
      return new Response(
        JSON.stringify({
          error: 'Offline',
          code: 'OFFLINE',
          message: 'You appear to be offline. Please check your connection.'
        }),
        {
          status: 503,
          headers: { 'Content-Type': 'application/json' }
        }
      );
    }

    throw error;
  }
}

/**
 * Stale-while-revalidate: Return cache immediately, update in background
 * Best for: HTML pages where freshness is desired but not critical
 */
async function staleWhileRevalidate(request, cacheName) {
  const cache = await caches.open(cacheName);
  const cachedResponse = await cache.match(request);

  const fetchPromise = fetch(request)
    .then((response) => {
      if (response.ok) {
        cache.put(request, response.clone());
      }
      return response;
    })
    .catch(() => cachedResponse || caches.match('/offline.html'));

  return cachedResponse || fetchPromise;
}

/**
 * Fetch and cache helper
 */
async function fetchAndCache(request, cacheName) {
  try {
    const response = await fetch(request);

    if (response.ok) {
      const cache = await caches.open(cacheName);
      cache.put(request, response.clone());
    }

    return response;
  } catch (error) {
    console.error('[SW] Fetch failed:', request.url, error);
    throw error;
  }
}

/**
 * Background sync for offline actions
 */
self.addEventListener('sync', (event) => {
  console.log('[SW] Background sync:', event.tag);

  if (event.tag === 'sync-claims') {
    event.waitUntil(syncPendingClaims());
  }
});

/**
 * Sync pending license claims when back online
 */
async function syncPendingClaims() {
  const pendingClaims = await getPendingClaims();

  for (const claim of pendingClaims) {
    try {
      const response = await fetch('/api/v1/licenses/claim', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(claim),
      });

      if (response.ok) {
        await removePendingClaim(claim.id);
      }
    } catch (error) {
      console.error('[SW] Failed to sync claim:', error);
    }
  }
}

/**
 * IndexedDB helpers for offline queue
 */
async function getPendingClaims() {
  // Placeholder - would use IndexedDB in production
  return [];
}

async function removePendingClaim(id) {
  // Placeholder - would use IndexedDB in production
}

/**
 * Push notification handler
 */
self.addEventListener('push', (event) => {
  if (!event.data) return;

  const data = event.data.json();

  const options = {
    body: data.body || 'New notification from UNO',
    icon: '/icons/icon-192.png',
    badge: '/icons/badge-72.png',
    vibrate: [100, 50, 100],
    data: {
      url: data.url || '/',
    },
    actions: data.actions || [],
  };

  event.waitUntil(
    self.registration.showNotification(data.title || 'UNO', options)
  );
});

/**
 * Notification click handler
 */
self.addEventListener('notificationclick', (event) => {
  event.notification.close();

  const url = event.notification.data?.url || '/';

  event.waitUntil(
    clients.matchAll({ type: 'window' })
      .then((clientList) => {
        // Focus existing window if open
        for (const client of clientList) {
          if (client.url === url && 'focus' in client) {
            return client.focus();
          }
        }
        // Open new window
        if (clients.openWindow) {
          return clients.openWindow(url);
        }
      })
  );
});

console.log('[SW] Service worker loaded');
