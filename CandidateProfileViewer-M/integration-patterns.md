# Integration Patterns — Candidate Profile Viewer

## Description
Defines how the Candidate Profile Viewer modules communicate with each other and with external systems, using synchronous request-response patterns (primary) with caching and fallback strategies to handle LinkedIn API constraints.

## Service Communication Overview

```mermaid
graph LR
    subgraph ClientLayer["Client Layer"]
        UI[Recruiter Web App]
    end

    subgraph Gateway["API Gateway"]
        GW[NGINX / Traefik<br/>JWT + Rate Limit]
    end

    subgraph Services["Application Services"]
        PSS[Profile Search Service]
        LIS[LinkedIn Integration Service]
        PCS[Profile Cache Service]
    end

    subgraph DataStores["Data Stores"]
        PG[(PostgreSQL<br/>Candidate Links)]
        ES[(Elasticsearch<br/>Name Index)]
        RD[(Redis<br/>Profile Cache)]
    end

    subgraph External["External Systems"]
        LI[LinkedIn API]
        ATS[ATS / CRM]
    end

    UI -->|HTTPS/JSON| GW
    GW -->|Route| PSS

    PSS -->|Fuzzy search| ES
    PSS -->|Read/Write links| PG
    PSS -->|Check cache| PCS
    PSS -->|Search LinkedIn| LIS

    PCS -->|GET/SET profiles| RD
    PCS -->|Refresh stale| LIS

    LIS -->|OAuth2 + REST| LI
    LIS -->|Token/quota state| RD

    PSS -->|Sync candidates| ATS
```

## End-to-End Search Flow (Cache Hit)

```mermaid
sequenceDiagram
    participant Recruiter
    participant UI as Web App
    participant GW as API Gateway
    participant PSS as Profile Search Service
    participant ES as Elasticsearch
    participant PCS as Cache Service
    participant Redis as Redis Cache

    Recruiter->>UI: Types "Marie Dupont" + Enter
    UI->>GW: GET /api/search?name=Marie+Dupont
    GW->>GW: Validate JWT, check rate limit
    GW->>PSS: Forward search request

    PSS->>ES: Fuzzy search "Marie Dupont"
    ES-->>PSS: Match: candidate-456 (linked to linkedin:abc123)

    PSS->>PCS: GET profile for linkedin:abc123
    PCS->>Redis: GET profile:abc123
    Redis-->>PCS: Cached profile JSON (TTL: 36h remaining)
    PCS-->>PSS: Profile data (cache hit)

    PSS-->>GW: 200 OK {profile: {...}, source: "cache"}
    GW-->>UI: 200 OK
    UI-->>Recruiter: Display LinkedIn profile
```

## End-to-End Search Flow (Cache Miss + LinkedIn API)

```mermaid
sequenceDiagram
    participant Recruiter
    participant UI as Web App
    participant GW as API Gateway
    participant PSS as Profile Search Service
    participant ES as Elasticsearch
    participant PCS as Cache Service
    participant LIS as LinkedIn Integration
    participant LinkedIn as LinkedIn API
    participant Redis as Redis Cache
    participant DB as PostgreSQL

    Recruiter->>UI: Types "Jean Martin" + Enter
    UI->>GW: GET /api/search?name=Jean+Martin
    GW->>PSS: Forward search request

    PSS->>ES: Fuzzy search "Jean Martin"
    ES-->>PSS: No confirmed link found

    PSS->>PCS: Check cache (no linkedinId known)
    PCS-->>PSS: CACHE_MISS

    PSS->>LIS: Search LinkedIn for "Jean Martin"
    LIS->>LIS: Check rate limit (42/100 daily calls used)
    LIS->>LinkedIn: GET /people-search?name=Jean+Martin&company=...
    LinkedIn-->>LIS: 3 matching profiles

    LIS->>LIS: Map to canonical schema
    LIS-->>PSS: 3 candidate profiles

    PSS->>PSS: Rank results (name similarity, location match)
    PSS-->>GW: 200 OK {results: [3 profiles], disambiguation: true}
    GW-->>UI: 200 OK
    UI-->>Recruiter: Display disambiguation cards (3 candidates)

    Recruiter->>UI: Clicks "Jean Martin — CTO at TechCorp"
    UI->>GW: POST /api/candidates/link {candidateId: 789, linkedinId: xyz456}
    GW->>PSS: Confirm link

    PSS->>DB: INSERT candidate_linkedin_link (789, xyz456)
    PSS->>PCS: Cache profile for xyz456
    PCS->>Redis: SET profile:xyz456 (TTL: 48h)

    PSS-->>GW: 201 Created
    GW-->>UI: 201 — Link confirmed
    UI-->>Recruiter: Display full profile (Jean Martin — CTO at TechCorp)
```

## Missing Profile Flow (Candidate Not on LinkedIn)

```mermaid
sequenceDiagram
    participant Recruiter
    participant UI as Web App
    participant GW as API Gateway
    participant PSS as Profile Search Service
    participant ES as Elasticsearch
    participant LIS as LinkedIn Integration
    participant LinkedIn as LinkedIn API
    participant DB as PostgreSQL

    Recruiter->>UI: Types "Pierre Legrand" + Enter
    UI->>GW: GET /api/search?name=Pierre+Legrand
    GW->>PSS: Forward search request

    PSS->>ES: Fuzzy search "Pierre Legrand"
    ES-->>PSS: No local match

    PSS->>LIS: Search LinkedIn for "Pierre Legrand"
    LIS->>LinkedIn: GET /people-search?name=Pierre+Legrand
    LinkedIn-->>LIS: 200 OK {results: []}

    LIS-->>PSS: NOT_FOUND (zero results)

    PSS->>DB: INSERT search_attempt (name="Pierre Legrand", result=NO_PROFILE, timestamp)
    PSS-->>GW: 200 OK {results: [], noProfileFound: true}
    GW-->>UI: 200 OK
    UI-->>Recruiter: "No LinkedIn profile found for Pierre Legrand"

    Note over UI: UI offers manual linking option
    Recruiter->>UI: Pastes LinkedIn URL manually
    UI->>GW: POST /api/candidates/manual-link {name: "Pierre Legrand", url: "linkedin.com/in/pierrelegrand"}
    GW->>PSS: Manual link request
    PSS->>LIS: Fetch profile by URL
    LIS->>LinkedIn: GET /profile/pierrelegrand
    LinkedIn-->>LIS: 200 OK {profile data}
    LIS-->>PSS: Profile retrieved
    PSS->>DB: INSERT candidate_linkedin_link
    PSS-->>GW: 201 Created
    GW-->>UI: Profile linked successfully
    UI-->>Recruiter: Display profile
```

## External System Integration

```mermaid
graph TB
    subgraph CandidateProfileViewer["Candidate Profile Viewer — System Boundary"]
        PSS[Profile Search Service]
        LIS[LinkedIn Integration Service]
        PCS[Profile Cache Service]
    end

    subgraph External["External Systems"]
        LIAPI[LinkedIn Talent Solutions API<br/>HTTPS/OAuth2]
        ATS[ATS / CRM<br/>REST/JSON]
        IDP[Identity Provider<br/>OAuth2/OIDC]
    end

    LIS -->|People Search<br/>Profile Retrieval<br/>~100 calls/day| LIAPI
    PSS -->|Read candidate records<br/>Write profile links| ATS
    PSS -->|Validate recruiter token| IDP
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Cache-Aside** | Check Redis before calling LinkedIn API | Reduce API calls; respect rate limits; improve latency |
| **Circuit Breaker** | LinkedIn Integration → LinkedIn API | Fail fast when LinkedIn is unavailable; prevent cascading failures |
| **Retry + Exponential Backoff** | Transient LinkedIn API errors (5xx, timeout) | Handle temporary failures without overwhelming the API |
| **Rate Limiting (Client-Side)** | LinkedIn Integration tracks daily/minute budget | Stay within LinkedIn's API quota; queue low-priority requests |
| **Request-Response (Sync)** | All service-to-service calls | Simple search UX requires synchronous response; low throughput doesn't need async |
| **Graceful Degradation** | Missing profile → structured "not found" response | Recruiter gets clear feedback instead of errors |
| **Manual Fallback** | Recruiter pastes LinkedIn URL directly | Handles edge cases where automated search fails |
| **Write-Through Cache** | On profile fetch, write to cache immediately | Ensures subsequent lookups hit cache |
| **Background Refresh** | Pre-fetch profiles approaching TTL expiry | Avoid cache misses during business hours |
| **API Gateway** | All client-to-service calls | Centralized auth, rate limiting, routing |

## Data Flow Metrics

| Stage | Throughput | Latency (p99) |
|-------|-----------|---------------|
| Recruiter → API Gateway | ~50 req/min (peak) | < 100ms |
| API Gateway → Profile Search | ~50 req/min | < 20ms (routing) |
| Profile Search → Elasticsearch | ~50 queries/min | < 50ms |
| Profile Search → Cache (hit) | ~40 req/min (80% hit rate) | < 10ms |
| Profile Search → LinkedIn (miss) | ~10 req/min | < 2s |
| LinkedIn API response | ~100 calls/day (quota) | 500ms–2s |
| End-to-end (cache hit) | — | < 200ms |
| End-to-end (cache miss) | — | < 3s |
