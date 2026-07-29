# Security Architecture Overlay — Candidate Profile Viewer

## Description
Cross-cutting security architecture showing network zones, authentication flows, encryption boundaries, access control enforcement points, and data privacy controls for the Candidate Profile Viewer system.

## Network Security Zones

```mermaid
graph TB
    subgraph Internet["Internet Zone"]
        browser[Recruiter Browser<br/>React SPA]
    end

    subgraph DMZ["DMZ — Perimeter Zone"]
        waf[Web Application Firewall<br/>OWASP rules, bot detection]
        lb[Load Balancer<br/>SSL termination]
    end

    subgraph AppZone["Application Zone"]
        gw[API Gateway<br/>JWT validation, rate limiting]

        subgraph PublicServices["Public-Facing Services"]
            profileSearch[Profile Search Service]
            cacheSvc[Profile Cache Service]
        end

        subgraph InternalServices["Internal Services — No Direct External Access"]
            linkedinSvc[LinkedIn Integration Service<br/>OAuth2 credentials, API calls]
        end
    end

    subgraph DataZone["Data Zone — No Direct External Access"]
        subgraph Search["Search Layer"]
            es[(Elasticsearch<br/>Candidate names only, no PII)]
        end

        subgraph Persistence["Persistence Layer"]
            candidateDb[(Candidate Index DB<br/>Links, search history)]
            auditDb[(Audit Log<br/>Append-only)]
        end

        subgraph Cache["Cache Layer"]
            redis[(Redis Cluster<br/>Cached profiles, tokens)]
        end
    end

    subgraph ExternalOut["Outbound External Zone — Whitelisted"]
        linkedinApi[LinkedIn API<br/>api.linkedin.com]
        ats[ATS / CRM]
        idp[Identity Provider]
    end

    browser --> waf
    waf --> lb
    lb --> gw
    gw --> PublicServices
    PublicServices --> InternalServices
    PublicServices --> DataZone
    InternalServices --> Cache
    InternalServices --> linkedinApi
    profileSearch --> ats
    gw --> idp
```

## Authentication & Authorization Flow

```mermaid
sequenceDiagram
    participant Recruiter
    participant App as Recruiter Web App
    participant GW as API Gateway
    participant IDP as Identity Provider (Auth0/Keycloak)
    participant PSS as Profile Search Service

    Recruiter->>App: Login (email + password)
    App->>IDP: POST /oauth/authorize (PKCE flow)
    IDP->>IDP: Validate credentials, enforce MFA
    IDP-->>App: Authorization code
    App->>IDP: POST /oauth/token (code + code_verifier)
    IDP-->>App: Access token (JWT, 30min) + Refresh token (7 days)

    Note over App: JWT contains: recruiterId, email, role (recruiter/senior/admin), agencyId

    Recruiter->>App: Search "Marie Dupont"
    App->>GW: GET /api/search?name=Marie+Dupont + Bearer {JWT}
    GW->>GW: Validate JWT signature (RS256 via JWKS)
    GW->>GW: Check token expiry
    GW->>GW: Extract role → enforce rate limit

    alt Valid token + authorized role
        GW->>PSS: Forward + X-Recruiter-Id, X-Agency-Id headers
        PSS->>PSS: Process search (role check: recruiter can search)
        PSS-->>GW: 200 OK {profile results}
        GW-->>App: 200 OK
    else Token expired
        GW-->>App: 401 Unauthorized (trigger silent refresh)
        App->>IDP: POST /oauth/token (refresh_token)
        IDP-->>App: New access token
    else Insufficient permissions
        GW-->>App: 403 Forbidden
    end
```

## RBAC Model

```mermaid
graph LR
    subgraph Roles
        R1[RECRUITER]
        R2[SENIOR_RECRUITER]
        R3[AGENCY_ADMIN]
    end

    subgraph Permissions
        P1[profile:search]
        P2[profile:view]
        P3[profile:link]
        P4[profile:manual-link]
        P5[profile:refresh]
        P6[profile:bulk-search]
        P7[system:view-analytics]
        P8[system:manage-api-keys]
        P9[system:manage-users]
        P10[audit:view-logs]
    end

    R1 --> P1
    R1 --> P2
    R1 --> P3

    R2 --> P1
    R2 --> P2
    R2 --> P3
    R2 --> P4
    R2 --> P5
    R2 --> P6

    R3 --> P1
    R3 --> P2
    R3 --> P3
    R3 --> P4
    R3 --> P5
    R3 --> P6
    R3 --> P7
    R3 --> P8
    R3 --> P9
    R3 --> P10
```

## Role-Based Access Control

| Role | Profile Search | Manual Link | Force Refresh | Bulk Search | Admin Panel | Rate Limit |
|------|---------------|-------------|---------------|-------------|-------------|------------|
| **Recruiter** | Yes | No | No | No | No | 30 req/min |
| **Senior Recruiter** | Yes | Yes | Yes | Yes (10/batch) | No | 60 req/min |
| **Agency Admin** | Yes | Yes | Yes | Yes (50/batch) | Yes | 120 req/min |

## Encryption Strategy

| Layer | Mechanism | Scope |
|-------|-----------|-------|
| **In transit (external)** | TLS 1.3 | All browser ↔ API communication |
| **In transit (internal)** | mTLS | Service-to-service within Application Zone |
| **In transit (LinkedIn)** | TLS 1.3 + OAuth2 bearer | All outbound LinkedIn API calls |
| **At rest (database)** | AES-256 (TDE) | PostgreSQL — candidate links, search history |
| **At rest (cache)** | Redis AUTH + TLS | Cached profiles encrypted in transit; evicted by TTL |
| **OAuth2 tokens** | Encrypted at rest in Redis | LinkedIn API tokens (agency credentials) |
| **Secrets management** | HashiCorp Vault / K8s Secrets | API keys, DB credentials, JWT signing keys |
| **Elasticsearch** | TLS + authentication | Search queries and index data encrypted in transit |

## LinkedIn API Security Controls

| Concern | Mitigation |
|---------|-----------|
| **OAuth2 token exposure** | Tokens stored encrypted in Redis; never exposed to frontend; rotated before expiry |
| **Rate limit exhaustion** | Client-side quota tracking; queue + priority system; alert admin at 80% usage |
| **API credential leak** | Secrets in Vault; never in code/config; rotation every 90 days |
| **Data scope (LinkedIn ToS)** | Only fetch fields permitted by Talent Solutions agreement; no scraping |
| **Profile data retention** | Cached profiles auto-expire (TTL); no permanent storage of LinkedIn data |

## Security Controls Summary

| Threat | Mitigation |
|--------|-----------|
| **Brute-force login** | MFA enforced; account lockout after 5 failed attempts; rate limiting |
| **JWT token theft** | Short-lived tokens (30min); refresh rotation; PKCE for SPA |
| **Unauthorized profile access** | RBAC enforced at API Gateway; row-level recruiter isolation |
| **LinkedIn credential compromise** | Encrypted storage; auto-rotation; single agency-level key (not per-user) |
| **SQL injection** | Parameterized queries; ORM layer; input validation on search terms |
| **XSS in profile display** | Content Security Policy; HTML sanitization of LinkedIn data before render |
| **SSRF via manual LinkedIn URL** | URL validation (must match linkedin.com/in/* pattern); no arbitrary URL fetch |
| **DDoS on search endpoint** | WAF + rate limiting at gateway; per-recruiter quotas |
| **Insider threat** | All searches logged to append-only audit DB; admin actions require MFA step-up |
| **Data breach (cached profiles)** | Profiles are non-sensitive business data; TTL ensures auto-deletion; no passwords stored |

## Compliance & Data Privacy

| Requirement | Implementation |
|-------------|---------------|
| **GDPR — Data minimization** | Cache only necessary profile fields (name, title, company, skills, photo URL); no private data |
| **GDPR — Right to erasure** | Candidate can request deletion → purge from cache + DB + search index |
| **GDPR — Lawful basis** | Legitimate interest (recruitment); documented in privacy policy |
| **LinkedIn ToS compliance** | Use only official API; no scraping; respect rate limits; display attribution |
| **Audit trail** | All profile searches, link confirmations, and admin actions logged immutably |
| **Data retention** | Cached profiles: 72h max; Search history: 1 year; Audit logs: 3 years |
| **Breach notification** | Automated detection; < 72h disclosure SLA per GDPR Art. 33 |
