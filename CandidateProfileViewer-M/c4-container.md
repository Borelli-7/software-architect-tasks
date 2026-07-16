# C4 Container Diagram — Candidate Profile Viewer

## Description
Shows the high-level runtime containers that compose the Candidate Profile Viewer system — applications, services, databases, cache, and search infrastructure.

```mermaid
C4Container
    title Container Diagram — Candidate Profile Viewer

    Person(recruiter, "Recruiter", "Searches candidates and views LinkedIn profiles")

    System_Boundary(profileViewer, "Candidate Profile Viewer") {
        Container(webApp, "Recruiter Web App", "React SPA", "Browser-based search interface — name input, disambiguation cards, profile display, manual linking")
        Container(apiGateway, "API Gateway", "NGINX/Traefik", "Routes requests, JWT validation, rate limiting, SSL termination")

        Container(profileSearchSvc, "Profile Search Service", "Node.js/Express", "Handles search queries, checks internal index, orchestrates LinkedIn lookups, returns ranked results")
        Container(linkedinSvc, "LinkedIn Integration Service", "Node.js/Express", "Manages OAuth2 tokens, executes LinkedIn API calls, maps responses, handles rate limits and missing profiles")
        Container(cacheSvc, "Profile Cache Service", "Node.js/Express", "Caches retrieved profiles with TTL, serves cached data, triggers background refresh")

        ContainerDb(candidateDb, "Candidate Index DB", "PostgreSQL", "Confirmed candidate-LinkedIn links, search history, recruiter activity log")
        Container(searchIndex, "Candidate Search Index", "Elasticsearch", "Fast fuzzy name matching against internal candidate records")
        Container(profileCache, "Profile Cache", "Redis", "Cached LinkedIn profiles, rate limit counters, token store")
    }

    System_Ext(linkedinApi, "LinkedIn API", "Talent Solutions REST API")
    System_Ext(ats, "ATS / CRM", "Existing recruitment system")
    System_Ext(authProvider, "Identity Provider", "OAuth2/OIDC")

    Rel(recruiter, webApp, "Uses", "HTTPS")
    Rel(webApp, apiGateway, "API calls", "HTTPS/JSON")

    Rel(apiGateway, profileSearchSvc, "Routes search requests", "HTTPS")
    Rel(apiGateway, cacheSvc, "Routes cache requests", "HTTPS")

    Rel(profileSearchSvc, searchIndex, "Fuzzy name search", "HTTPS/REST")
    Rel(profileSearchSvc, candidateDb, "Reads/writes candidate links", "SQL/TLS")
    Rel(profileSearchSvc, linkedinSvc, "Requests LinkedIn profile search", "HTTPS/internal")
    Rel(profileSearchSvc, cacheSvc, "Checks cache before LinkedIn call", "HTTPS/internal")

    Rel(linkedinSvc, linkedinApi, "Searches/fetches profiles", "HTTPS/OAuth2")
    Rel(linkedinSvc, profileCache, "Reads/writes tokens, rate counters", "Redis/TLS")

    Rel(cacheSvc, profileCache, "Reads/writes cached profiles", "Redis/TLS")
    Rel(cacheSvc, linkedinSvc, "Triggers background refresh", "HTTPS/internal")

    Rel(profileSearchSvc, ats, "Syncs candidate records", "REST/JSON")
    Rel(apiGateway, authProvider, "Validates JWT tokens", "HTTPS/OIDC")
```

## Container Responsibilities

| Container | Scaling Strategy | Data Ownership |
|-----------|-----------------|----------------|
| Recruiter Web App | CDN for static assets; edge caching | None (stateless SPA) |
| API Gateway | Horizontal, multi-instance behind LB | None (stateless) |
| Profile Search Service | Horizontal (2–4 instances); stateless | Search orchestration logic |
| LinkedIn Integration Service | Single instance with queue (rate limit gated) | OAuth2 tokens, API quota state |
| Profile Cache Service | Horizontal (read-heavy) | Cache policy, TTL management |
| Candidate Index DB (PostgreSQL) | Primary + read replica | Candidate-LinkedIn links, search history |
| Candidate Search Index (Elasticsearch) | 3-node cluster, sharded by name prefix | Indexed candidate names, metadata |
| Profile Cache (Redis) | Clustered, sharded by candidate ID | Cached profiles (TTL: 24–72h), rate counters |
