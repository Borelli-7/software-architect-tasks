# Security Architecture Overlay — Stock Price Alerts

## Description
Cross-cutting security architecture showing network zones, authentication flows, encryption boundaries, and access control enforcement points for the Stock Price Alert system.

## Network Security Zones

```mermaid
graph TB
    subgraph Internet["Internet Zone"]
        browser[Web/Mobile App]
        feeds[Market Data Feeds]
    end

    subgraph DMZ["DMZ — Perimeter Zone"]
        waf[Web Application Firewall]
        lb[Load Balancer]
        feedProxy[Feed Ingress Proxy<br/>IP whitelisting, protocol inspection]
    end

    subgraph AppZone["Application Zone"]
        gw[API Gateway<br/>JWT validation, rate limiting]

        subgraph PublicServices["Public-Facing Services"]
            alertApi[Alert Management API]
            userSvc[User Service]
        end

        subgraph InternalServices["Internal Processing Services"]
            marketDataSvc[Market Data Service]
            ruleEngine[Alert Rule Engine]
            notifSvc[Notification Service]
        end
    end

    subgraph DataZone["Data Zone — No Direct External Access"]
        subgraph Streaming["Streaming Layer"]
            kafka[Kafka Cluster<br/>mTLS between brokers]
            redis[Redis Cluster<br/>AUTH + TLS]
        end

        subgraph Persistence["Persistence Layer"]
            alertDb[(Alert Rules DB<br/>Encrypted at rest)]
            userDb[(User/Preferences DB<br/>PII encrypted)]
            auditDb[(Audit Log<br/>Append-only, immutable)]
        end
    end

    subgraph ExternalOut["Outbound External Zone"]
        email[Email Gateway]
        sms[SMS Gateway]
        push[Push Service]
    end

    browser --> waf
    waf --> lb
    lb --> gw
    feeds --> feedProxy
    feedProxy --> marketDataSvc

    gw --> PublicServices
    PublicServices --> Streaming
    InternalServices --> Streaming
    InternalServices --> Persistence
    PublicServices --> Persistence

    notifSvc --> email
    notifSvc --> sms
    notifSvc --> push
```

## Authentication & Authorization Flow

```mermaid
sequenceDiagram
    participant User
    participant App as Web/Mobile App
    participant GW as API Gateway
    participant Auth as Auth Service (OAuth2)
    participant API as Alert Management API

    User->>App: Login (email + password)
    App->>Auth: POST /oauth/token (grant_type=password)
    Auth->>Auth: Validate credentials, check MFA if enabled
    Auth-->>App: Access token (JWT, 15min) + Refresh token (7 days)

    Note over App: JWT contains: userId, email, tier (basic/premium), permissions

    User->>App: Create alert rule
    App->>GW: POST /alerts + Bearer {JWT}
    GW->>GW: Validate JWT signature (RS256)
    GW->>GW: Check token expiry
    GW->>GW: Extract tier → enforce rate limit (basic: 10 alerts, premium: unlimited)
    
    alt Valid token + within limits
        GW->>API: Forward + X-User-Id, X-User-Tier headers
        API->>API: Validate rule (symbol exists, valid condition)
        API-->>GW: 201 Created
        GW-->>App: 201 Created
    else Rate limit exceeded
        GW-->>App: 429 Too Many Requests
    else Invalid/expired token
        GW-->>App: 401 Unauthorized
    end
```

## RBAC Model

```mermaid
graph LR
    subgraph Roles
        R1[CUSTOMER_BASIC]
        R2[CUSTOMER_PREMIUM]
        R3[SYSTEM_ADMIN]
    end

    subgraph Permissions
        P1[alert:create]
        P2[alert:read:own]
        P3[alert:update:own]
        P4[alert:delete:own]
        P5[alert:unlimited]
        P6[notification:configure]
        P7[system:monitor]
        P8[system:config]
        P9[user:manage]
        P10[alert:read:all]
    end

    R1 --> P1
    R1 --> P2
    R1 --> P3
    R1 --> P4
    R1 --> P6

    R2 --> P1
    R2 --> P2
    R2 --> P3
    R2 --> P4
    R2 --> P5
    R2 --> P6

    R3 --> P7
    R3 --> P8
    R3 --> P9
    R3 --> P10
```

## Tier-Based Access Control

| Tier | Max Alerts | Notification Channels | Alert Types | Rate Limit |
|------|-----------|----------------------|-------------|------------|
| **Basic** | 10 active | Email only | Price above/below | 5 req/min |
| **Premium** | Unlimited | Email + SMS + Push | All condition types | 60 req/min |
| **Admin** | N/A | N/A | Full system access | 120 req/min |

## Encryption Strategy

| Layer | Mechanism | Scope |
|-------|-----------|-------|
| **In transit (external)** | TLS 1.3 | All client ↔ API communication |
| **In transit (internal)** | mTLS | Service-to-service, Kafka broker-to-broker |
| **At rest (database)** | AES-256 (Transparent Data Encryption) | All PostgreSQL databases |
| **At rest (PII fields)** | Application-level field encryption | Email, phone, device tokens in User DB |
| **Kafka events** | Envelope encryption | AlertTriggered events contain userId — encrypted payload |
| **Redis cache** | AUTH + TLS | In-transit encryption; no PII stored in cache |
| **Secrets management** | HashiCorp Vault / K8s Secrets | API keys, DB credentials, JWT signing keys |

## Security Controls Summary

| Threat | Mitigation |
|--------|-----------|
| **Brute-force login** | Rate limiting + account lockout after 5 failed attempts + MFA |
| **JWT token theft** | Short-lived tokens (15min), refresh token rotation, token binding |
| **Alert enumeration** | Users can only access their own alerts (row-level security) |
| **Notification spam** | Per-user rate limits, cooldown periods, quiet hours |
| **Market data tampering** | Feed ingress via whitelisted IPs only; protocol-level validation |
| **SQL injection** | Parameterized queries, ORM layer, input validation |
| **DDoS on API** | WAF + rate limiting at gateway + auto-scaling |
| **Insider threat** | Admin actions logged to append-only audit DB; least-privilege IAM |
| **Data breach** | Field-level encryption for PII; encryption keys rotated quarterly |

## Compliance & Audit

| Requirement | Implementation |
|-------------|---------------|
| **Audit trail** | All alert CRUD, trigger events, and admin actions logged to immutable audit DB |
| **Data retention** | Alert history: 2 years; audit logs: 5 years; price data: 90 days |
| **Right to deletion (GDPR)** | User deletion cascades to alerts, preferences, and notification history; audit logs anonymized |
| **Breach notification** | Automated detection pipeline; < 72h disclosure SLA |
