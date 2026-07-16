# Candidate Profile Viewer Architecture Plan — Executive Recruitment Agency

## Executive Summary

A software module for an executive recruitment agency that allows recruiters to view a candidate's LinkedIn profile by simply typing the candidate's name into a browser-based interface. The system integrates with LinkedIn's API (Talent Solutions) to retrieve and display profile data, eliminating the need for recruiters to log in to LinkedIn repeatedly. The architecture addresses name disambiguation (multiple candidates matching a search), graceful handling of candidates without LinkedIn profiles, and caching to respect API rate limits and improve response times.

---

## 1. Architectural Decisions

### 1.1 LinkedIn API Integration Strategy
| Decision | Rationale |
|----------|-----------|
| **LinkedIn Talent Solutions API (REST)** | Enterprise-grade access to profile data; agency-level OAuth2 credentials avoid per-recruiter login |
| **Agency-level OAuth2 token (client credentials)** | Single integration point; recruiters don't need individual LinkedIn accounts to search |
| **Profile data caching with TTL** | Reduces API calls, respects LinkedIn's rate limits (~100 req/day for some endpoints), improves latency |
| **Asynchronous profile enrichment** | Background refresh of cached profiles ensures freshness without blocking recruiter queries |

### 1.2 Search & Matching Strategy
| Decision | Rationale |
|----------|-----------|
| **Name-based search with disambiguation UI** | Candidate names may match multiple LinkedIn profiles; recruiter selects the correct one |
| **Fuzzy matching + filters (company, title, location)** | Improves precision when common names return multiple results |
| **Profile linking (candidate → LinkedIn URL)** | Once a recruiter confirms a match, the link is stored — no repeat disambiguation |
| **Internal candidate index** | Stores previously resolved candidates for instant lookup on subsequent searches |

### 1.3 Missing Profile Handling
| Decision | Rationale |
|----------|-----------|
| **Graceful fallback with clear UX messaging** | Not all candidates have LinkedIn profiles; system must inform recruiter without error |
| **Manual profile linking** | Recruiter can paste a LinkedIn URL directly if automated search fails |
| **Alternative data display** | Show internal ATS/CRM data when LinkedIn profile unavailable |
| **"No profile found" status tracking** | Persist the search attempt to avoid repeated API calls for non-existent profiles |

### 1.4 Security & Privacy Strategy
| Decision | Rationale |
|----------|-----------|
| **OAuth 2.0 + JWT for recruiter authentication** | Secure access; stateless token validation |
| **Role-based access (RBAC)** | Only authorized recruiters can search profiles |
| **GDPR-compliant data retention** | Cached profile data subject to TTL and deletion policies |
| **No credential storage for LinkedIn per-user** | Agency-level API key only; recruiters never provide LinkedIn passwords |

---

## 2. Technology Stack

| Layer | Technology | Justification |
|-------|-----------|---------------|
| Frontend | React SPA | Fast search UX, responsive profile display, browser-based |
| API Gateway | NGINX / Traefik | Rate limiting, JWT validation, SSL termination |
| Backend Services | Node.js (Express/Fastify) | Lightweight, async I/O for API proxy; fast JSON handling |
| LinkedIn Integration | LinkedIn Talent Solutions REST API | Official API for profile search and retrieval |
| Primary Database | PostgreSQL | Candidate-profile links, search history, recruiter data — ACID-compliant |
| Cache | Redis | Cached profiles, rate limit counters, session store |
| Internal Search | Elasticsearch | Fast fuzzy name matching against internal candidate index |
| Authentication | Auth0 / Keycloak | OAuth2 + JWT issuer for recruiter login |
| Monitoring | Prometheus + Grafana | API latency, cache hit rates, LinkedIn quota tracking |
| CI/CD | GitHub Actions | Automated testing and deployment |

---

## 3. Module Breakdown

### 3.1 Profile Search Service
- Accept recruiter search query (candidate name, optional filters)
- Check internal candidate index for previously linked profiles
- If not found locally, query LinkedIn API via Integration Service
- Return ranked list of matching profiles for disambiguation
- Store confirmed candidate-profile link for future instant retrieval

### 3.2 LinkedIn Integration Service
- Manage OAuth2 tokens (refresh, rotation, secure storage)
- Execute LinkedIn People Search API calls
- Fetch full profile data for a specific LinkedIn member
- Handle rate limiting (queue requests, respect quotas)
- Map LinkedIn response format to internal canonical schema
- Handle missing profiles (API returns empty results)

### 3.3 Profile Cache Service
- Cache retrieved LinkedIn profiles with configurable TTL (24–72 hours)
- Serve cached profiles for repeated lookups
- Background refresh of stale profiles before expiry
- Invalidation on manual recruiter request (force refresh)
- Track cache hit/miss rates for monitoring

### 3.4 Recruiter UI (Browser Module)
- Search bar for candidate name entry
- Disambiguation view (multiple results with preview cards)
- Full profile display (experience, education, skills, photo)
- "No profile found" state with manual linking option
- Profile refresh action and link history

### 3.5 Candidate Index Service
- Maintain internal index of known candidates (from ATS/CRM)
- Store confirmed LinkedIn profile links
- Sync candidate data from existing recruitment systems
- Support bulk import of candidate-profile mappings

---

## 4. Diagram Inventory

| # | Diagram | Format | File |
|---|---------|--------|------|
| 1 | C4 System Context | Mermaid | `c4-context.md` |
| 2 | C4 Container | Mermaid | `c4-container.md` |
| 3 | C4 Component — Profile Search | Mermaid | `c4-component-profile-search.md` |
| 4 | C4 Component — LinkedIn Integration | Mermaid | `c4-component-linkedin-integration.md` |
| 5 | C4 Component — Profile Cache | Mermaid | `c4-component-profile-cache.md` |
| 6 | Integration Patterns | Mermaid | `integration-patterns.md` |
| 7 | Security Architecture Overlay | Mermaid | `security-architecture.md` |

---

## 5. Execution Order

1. **Phase 1 — Context** → `c4-context.md` (system boundaries, actors, LinkedIn API, ATS integration)
2. **Phase 2 — Containers** → `c4-container.md` (services, databases, cache, search index)
3. **Phase 3 — Components** → Component-level diagrams for Profile Search, LinkedIn Integration, Profile Cache
4. **Phase 4 — Cross-cutting** → Integration patterns + security overlay
5. **Phase 5 — Verification** → Review consistency across diagrams, validate naming, confirm API constraints
