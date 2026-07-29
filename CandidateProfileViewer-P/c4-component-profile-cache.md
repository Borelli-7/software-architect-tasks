# C4 Component Diagram — Profile Cache Service

## Description
Internal components of the Profile Cache microservice, responsible for storing retrieved LinkedIn profiles with time-based expiry, serving cached data for repeated lookups, triggering background refresh of stale entries, and managing invalidation policies.

```mermaid
C4Component
    title Component Diagram — Profile Cache Service

    Container_Boundary(cacheSvc, "Profile Cache Service") {
        Component(cacheManager, "Cache Manager", "Orchestrator", "Routes read/write requests to cache — checks existence, freshness, and returns data or triggers fetch")
        Component(profileStore, "Profile Store", "Redis Client", "Reads and writes serialized profile data in Redis with structured keys (profile:{linkedinId})")
        Component(ttlPolicyEngine, "TTL Policy Engine", "Policy Evaluator", "Determines TTL per profile based on recency, recruiter access frequency, and profile completeness — 24h to 72h range")
        Component(backgroundRefresher, "Background Refresher", "Scheduled Worker", "Proactively refreshes profiles approaching TTL expiry — avoids cache misses during business hours")
        Component(invalidationHandler, "Invalidation Handler", "Event Listener", "Handles manual refresh requests from recruiters and bulk invalidation events (e.g., LinkedIn API schema change)")
        Component(metricsCollector, "Metrics Collector", "Instrumentation", "Tracks cache hit/miss rates, refresh counts, eviction rates, and storage utilization")
    }

    Container(profileCache, "Profile Cache", "Redis Cluster")
    Container(linkedinSvc, "LinkedIn Integration Service", "Node.js")
    Container(profileSearchSvc, "Profile Search Service", "Node.js")
    Container(apiGateway, "API Gateway", "NGINX")

    Rel(profileSearchSvc, cacheManager, "GET profile for candidate")
    Rel(cacheManager, profileStore, "Read profile:{linkedinId}")
    Rel(profileStore, profileCache, "GET/SET operations", "Redis/TLS")
    Rel(cacheManager, ttlPolicyEngine, "Determine TTL for new entry")
    Rel(ttlPolicyEngine, profileStore, "SET with computed TTL")
    Rel(cacheManager, profileSearchSvc, "Return cached profile or CACHE_MISS")
    Rel(backgroundRefresher, profileCache, "SCAN keys with TTL < threshold", "Redis/TLS")
    Rel(backgroundRefresher, linkedinSvc, "Refresh stale profile", "HTTPS")
    Rel(backgroundRefresher, profileStore, "UPDATE refreshed profile")
    Rel(apiGateway, invalidationHandler, "Manual refresh request from recruiter")
    Rel(invalidationHandler, profileStore, "DELETE profile:{linkedinId}")
    Rel(invalidationHandler, linkedinSvc, "Trigger immediate re-fetch", "HTTPS")
    Rel(metricsCollector, profileCache, "Monitor key count, memory usage", "Redis/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `ProfileCacheHit` | Cached profile served to Profile Search Service | Monitoring (hit rate metrics) |
| `ProfileCacheMiss` | No cached profile found, triggering LinkedIn fetch | Monitoring, LinkedIn Integration |
| `ProfileCacheRefreshed` | Background refresher updated a stale profile | Audit Log |
| `ProfileCacheInvalidated` | Recruiter requested manual refresh or bulk invalidation | Audit Log, LinkedIn Integration |
| `ProfileCacheEvicted` | Profile expired by TTL or memory pressure | Monitoring (eviction rate) |
| `CacheCapacityWarning` | Redis memory usage exceeds 80% threshold | Admin Alert |

## Data Flow Characteristics

| Metric | Value |
|--------|-------|
| Cache hit rate target | > 80% (most candidates searched repeatedly) |
| Default TTL | 48 hours (adjustable per policy) |
| TTL range | 24–72 hours based on access frequency and profile completeness |
| Background refresh window | Profiles with < 4h remaining TTL during business hours |
| Max cached profiles | ~50,000 (agency's active candidate pool) |
| Average profile size | 5–15 KB (serialized JSON) |
| Redis memory budget | ~500 MB (50K profiles × 10 KB avg) |
| Cache read latency | < 5ms (Redis cluster) |
