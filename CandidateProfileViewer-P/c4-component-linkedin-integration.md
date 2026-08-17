# C4 Component Diagram — LinkedIn Integration Service

## Description
Internal components of the LinkedIn Integration microservice, responsible for managing OAuth2 credentials, executing LinkedIn API calls, handling rate limits, mapping response data, and gracefully managing cases where candidates have no LinkedIn profile.

```mermaid
C4Component
    title Component Diagram — LinkedIn Integration Service

    Container_Boundary(linkedinSvc, "LinkedIn Integration Service") {
        Component(tokenManager, "OAuth2 Token Manager", "Auth Client", "Manages agency-level OAuth2 tokens — handles acquisition, refresh, secure storage, and rotation before expiry")
        Component(rateLimiter, "Rate Limiter", "Quota Guard", "Tracks LinkedIn API call budget (daily/minute limits); queues requests when approaching threshold; rejects when exhausted")
        Component(profileFetcher, "Profile Fetcher", "HTTP Client", "Executes LinkedIn People Search and Profile Retrieval API calls with proper headers and pagination")
        Component(responseMapper, "Response Mapper", "Transformer", "Maps LinkedIn's JSON response to internal canonical profile schema — normalizes fields, handles partial data")
        Component(fallbackHandler, "Fallback Handler", "Error Handler", "Manages scenarios where LinkedIn returns no results, 404, or rate-limit errors — returns structured 'not found' response")
        Component(retryEngine, "Retry Engine", "Resilience Layer", "Implements exponential backoff for transient LinkedIn API failures (5xx, timeouts); circuit breaker for sustained outages")
    }

    System_Ext(linkedinApi, "LinkedIn API", "Talent Solutions REST API")
    Container(profileCache, "Profile Cache", "Redis")
    Container(profileSearchSvc, "Profile Search Service", "Node.js")

    Rel(profileSearchSvc, rateLimiter, "Search/fetch request")
    Rel(rateLimiter, profileFetcher, "Approved request (within quota)")
    Rel(rateLimiter, fallbackHandler, "Rejected request (quota exhausted)")
    Rel(profileFetcher, tokenManager, "Get valid OAuth2 bearer token")
    Rel(tokenManager, profileCache, "Read/write token", "Redis/TLS")
    Rel(profileFetcher, linkedinApi, "GET /people-search, GET /profile/{id}", "HTTPS/OAuth2")
    Rel(profileFetcher, retryEngine, "Transient failure (5xx, timeout)")
    Rel(retryEngine, profileFetcher, "Retry with backoff")
    Rel(retryEngine, fallbackHandler, "Max retries exceeded / circuit open")
    Rel(profileFetcher, responseMapper, "Raw LinkedIn JSON response")
    Rel(responseMapper, profileSearchSvc, "Canonical profile(s) or empty result")
    Rel(fallbackHandler, profileSearchSvc, "Structured error: NOT_FOUND / RATE_LIMITED / UNAVAILABLE")
    Rel(rateLimiter, profileCache, "Read/write quota counters", "Redis/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `LinkedInProfileFetched` | Successful profile retrieval from LinkedIn API | Profile Cache (store), Analytics |
| `LinkedInSearchExecuted` | People Search API call completed | Monitoring (quota tracking) |
| `LinkedInProfileNotFound` | LinkedIn returns 0 results for a candidate search | Profile Search (display "not found"), Candidate Index |
| `LinkedInRateLimitHit` | Daily/minute quota reached | Admin Alert, Queue Manager |
| `LinkedInApiUnavailable` | Circuit breaker opened after repeated failures | Admin Alert, Fallback Handler |
| `OAuthTokenRefreshed` | OAuth2 token successfully refreshed before expiry | Audit Log |

## Data Flow Characteristics

| Metric | Value |
|--------|-------|
| LinkedIn API rate limit | ~100 calls/day (varies by API tier; Talent Solutions allows more) |
| People Search latency | 500ms–2s (LinkedIn side) |
| Profile Retrieval latency | 300ms–1s (LinkedIn side) |
| Token refresh frequency | Every 60 days (LinkedIn OAuth2 token lifetime) |
| Circuit breaker threshold | Opens after 5 consecutive failures; half-open retry after 60s |
| Retry policy | Exponential backoff: 1s, 2s, 4s (max 3 retries) |
| Fallback response time | < 50ms (immediate structured error) |
