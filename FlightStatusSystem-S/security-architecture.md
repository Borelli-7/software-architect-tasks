# Security Architecture Overlay — Flight Status Distribution System

## Description
Cross-cutting security architecture showing network zones, authentication flows, encryption boundaries, and access control enforcement points for the Flight Status Distribution system. Special emphasis on protecting the sensitive radar/ATC feed connection and managing multi-tenant consumer access.

## Network Security Zones

```mermaid
graph TB
    subgraph Internet["Internet Zone"]
        consumers[External Consumers<br/>Airlines, OTAs, Hotels, Apps]
        adminBrowser[Admin Browser]
    end

    subgraph DMZ["DMZ — Perimeter Zone"]
        waf[Web Application Firewall]
        lb[Load Balancer]
        wsGateway[WebSocket/SSE Gateway<br/>Connection termination, auth validation]
    end

    subgraph AppZone["Application Zone"]
        gw[API Gateway<br/>API key validation, rate limiting, JWT check]

        subgraph PublicServices["Consumer-Facing Services"]
            consumerApi[Consumer REST API]
            distributionSvc[Distribution Service]
            adminDashboard[Admin Dashboard]
        end

        subgraph InternalServices["Internal Processing Services"]
            flightProcessor[Flight Status Processor]
            scheduleSvc[Schedule Service]
        end
    end

    subgraph RestrictedZone["Restricted Zone — No Internet Access"]
        subgraph Ingestion["Ingestion Layer"]
            ingestionSvc[Flight Data Ingestion Service<br/>mTLS, IP whitelist only]
        end

        subgraph Streaming["Streaming Layer"]
            kafka[Kafka Cluster<br/>mTLS between brokers]
            redis[Redis Cluster<br/>AUTH + TLS]
        end

        subgraph Persistence["Persistence Layer"]
            flightDb[(Flight Status DB<br/>Encrypted at rest)]
            consumerDb[(Consumer Registry DB<br/>API keys hashed)]
            auditDb[(Audit Log<br/>Append-only, immutable)]
        end
    end

    subgraph ATCZone["ATC Zone — Air-Gapped Network Bridge"]
        atcBridge[ATC Network Bridge<br/>Unidirectional data diode]
        centralATC[Central Radar/ATC System]
    end

    consumers --> waf
    waf --> lb
    lb --> wsGateway
    lb --> gw
    adminBrowser --> waf

    wsGateway --> distributionSvc
    gw --> PublicServices

    PublicServices --> Streaming
    InternalServices --> Streaming
    InternalServices --> Persistence
    PublicServices --> Persistence

    centralATC --> atcBridge
    atcBridge --> ingestionSvc
    ingestionSvc --> Streaming
```

## Authentication & Authorization Flow

```mermaid
sequenceDiagram
    participant Consumer
    participant GW as API Gateway
    participant Auth as Auth Service
    participant API as Consumer REST API
    participant DIST as Distribution Service

    Note over Consumer: Consumer registers and receives API key + OAuth credentials
    
    Consumer->>Auth: POST /oauth/token (client_credentials grant)
    Auth->>Auth: Validate client_id + client_secret
    Auth-->>Consumer: Access token (JWT, 1 hour) — contains: consumerId, tier, permissions

    Note over Consumer: REST API access
    Consumer->>GW: GET /flights/AA123 + Bearer {JWT} + X-API-Key
    GW->>GW: Validate JWT signature (RS256)
    GW->>GW: Validate API key matches JWT consumer
    GW->>GW: Check rate limit (tier-based)
    
    alt Valid token + within limits
        GW->>API: Forward + X-Consumer-Id, X-Consumer-Tier headers
        API->>API: Validate request (flight exists, consumer has access)
        API-->>GW: 200 OK
        GW-->>Consumer: 200 OK
    else Rate limit exceeded
        GW-->>Consumer: 429 Too Many Requests
    else Invalid/expired token
        GW-->>Consumer: 401 Unauthorized
    end

    Note over Consumer: WebSocket subscription
    Consumer->>DIST: WS Connect + Bearer {JWT} in handshake
    DIST->>DIST: Validate JWT, extract consumer tier + permissions
    DIST->>DIST: Register connection, apply subscription filters
    DIST-->>Consumer: Connected + subscription confirmed
    
    loop Push updates
        DIST->>Consumer: Flight status event (filtered by subscription)
    end
```

## RBAC Model

```mermaid
graph LR
    subgraph Roles
        R1[CONSUMER_BASIC]
        R2[CONSUMER_STANDARD]
        R3[CONSUMER_PREMIUM]
        R4[CONSUMER_PARTNER]
        R5[AIRPORT_ADMIN]
    end

    subgraph Permissions
        P1[flight:read:single]
        P2[flight:read:filtered]
        P3[flight:read:all]
        P4[subscription:create:flight]
        P5[subscription:create:airline]
        P6[subscription:create:wildcard]
        P7[websocket:connect]
        P8[sse:connect]
        P9[consumer:manage]
        P10[system:monitor]
        P11[system:config]
        P12[feed:health:read]
    end

    R1 --> P1
    R1 --> P4

    R2 --> P1
    R2 --> P2
    R2 --> P4
    R2 --> P5
    R2 --> P7
    R2 --> P8

    R3 --> P1
    R3 --> P2
    R3 --> P4
    R3 --> P5
    R3 --> P7
    R3 --> P8

    R4 --> P1
    R4 --> P2
    R4 --> P3
    R4 --> P4
    R4 --> P5
    R4 --> P6
    R4 --> P7
    R4 --> P8

    R5 --> P9
    R5 --> P10
    R5 --> P11
    R5 --> P12
```

## Consumer Tier Access Control

| Tier | API Rate Limit | Push Connections | Data Access | Subscription Filters |
|------|---------------|-----------------|-------------|---------------------|
| **Basic** | 60 req/min | None (REST only) | Single flight lookup | By flight ID only |
| **Standard** | 600 req/min | 10 WebSocket/SSE | Filtered queries (airline, route) | By airline, route, airport |
| **Premium** | 6,000 req/min | Unlimited | Full query access | All filters including time windows |
| **Partner** | Unlimited | Unlimited | Full feed + historical data | Wildcard — receives all events |
| **Admin** | Unlimited | N/A | Full system access | N/A |

## Encryption Strategy

| Layer | Mechanism | Scope |
|-------|-----------|-------|
| **In transit (external)** | TLS 1.3 | All consumer ↔ API/WebSocket communication |
| **In transit (ATC feed)** | mTLS + data diode | Ingestion ↔ Central ATC system (unidirectional) |
| **In transit (internal)** | mTLS | Service-to-service, Kafka broker-to-broker |
| **At rest (database)** | AES-256 (Transparent Data Encryption) | All PostgreSQL databases |
| **At rest (API keys)** | bcrypt hash + salt | Consumer API keys in registry DB |
| **Kafka events** | TLS in transit; no PII in flight data | Position events contain only flight operational data |
| **Redis cache** | AUTH + TLS | In-transit encryption; flight data only (no PII) |
| **Secrets management** | HashiCorp Vault / K8s Secrets | API keys, DB credentials, JWT signing keys, TLS certs |

## Security Controls Summary

| Threat | Mitigation |
|--------|-----------|
| **Unauthorized access to flight data** | API key + OAuth2 JWT double validation; tier-based access control |
| **ATC feed interception** | mTLS + data diode (unidirectional); air-gapped network bridge; IP whitelist |
| **Consumer impersonation** | API key bound to JWT claims; key rotation every 90 days |
| **DDoS on API/WebSocket** | WAF + rate limiting at gateway + auto-scaling; per-consumer connection limits |
| **Data poisoning (fake positions)** | Ingestion service validates against known flight plans; anomaly detection on positions |
| **Insider threat** | Admin actions logged to append-only audit DB; least-privilege IAM; no direct DB access |
| **WebSocket hijacking** | JWT validated on handshake; connection bound to consumer IP; periodic re-auth |
| **Feed outage manipulation** | Heartbeat monitoring detects outages; staleness indicators shown to consumers |
| **SQL injection** | Parameterized queries, ORM layer, input validation |
| **Consumer data leakage** | Each consumer sees only their subscribed subset; no cross-tenant data exposure |

## Compliance & Audit

| Requirement | Implementation |
|-------------|---------------|
| **Audit trail** | All consumer access, subscription changes, and admin actions logged to immutable audit DB |
| **Data retention** | Flight status: 90 days; position history: 30 days; audit logs: 5 years; consumer data: account lifetime |
| **Aviation data regulations** | Compliance with ICAO Annex 10 for surveillance data handling; local civil aviation authority requirements |
| **Consumer data agreement** | Each consumer signs data usage agreement specifying allowed redistribution and display rules |
| **Incident response** | Automated detection of feed anomalies; < 5 min escalation SLA for ATC connection loss |
| **Key rotation** | Consumer API keys: 90 days; JWT signing keys: 30 days; TLS certs: annual; mTLS certs: annual |
