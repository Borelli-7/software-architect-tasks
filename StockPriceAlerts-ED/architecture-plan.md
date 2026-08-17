# Stock Price Alerts Architecture Plan — Brokerage Firm

## Executive Summary

Real-time stock price alert system for a large brokerage firm serving thousands of retail customers. The architecture follows an event-driven, stream-processing approach to evaluate customer-defined alert rules against live market data feeds. Supports thousands of traded stocks, millions of alert rules, and multi-channel notification delivery (email, SMS, push).

---

## 1. Architectural Decisions

### 1.1 Real-Time Processing Strategy
| Decision | Rationale |
|----------|-----------|
| **Event-driven over polling** | Subscribe to price events for sub-second latency; avoids wasteful periodic queries |
| **Stream processing (Kafka Streams)** | Stateful rule evaluation at throughput of millions of events/second |
| **Partitioning by stock symbol** | Each price update evaluates only the rules for that ticker — O(1) lookup, not full-scan |
| **In-memory rule cache + CDC refresh** | Hot rules in memory; Change Data Capture propagates DB changes without restarts |

### 1.2 Scalability Strategy
| Decision | Rationale |
|----------|-----------|
| **Horizontal scaling via Kafka partitions** | Each partition handles a subset of symbols; add partitions = add throughput |
| **Stateless notification workers** | Scale independently based on alert volume spikes |
| **Redis price cache** | Latest prices available for on-demand queries without hitting the stream |
| **Read replicas for alert DB** | Customer-facing API reads don't compete with rule-engine writes |

### 1.3 Reliability Strategy
| Decision | Rationale |
|----------|-----------|
| **At-least-once delivery + idempotency keys** | No alerts lost; deduplication prevents customer spam |
| **Dead Letter Queues** | Failed notifications isolated for retry without blocking pipeline |
| **Circuit breaker on external gateways** | Protects system when email/SMS providers are degraded |
| **Alert cooldown periods** | Configurable per-rule cooldown prevents re-triggering (e.g., 1 hour) |

### 1.4 Security Strategy
| Decision | Rationale |
|----------|-----------|
| **OAuth 2.0 + JWT** | Standard auth for API access; stateless token validation |
| **TLS everywhere** | All traffic encrypted in transit (internal and external) |
| **AES-256 at rest** | Customer data and alert rules encrypted in database |
| **Rate limiting on API** | Prevent abuse of alert creation endpoints |

---

## 2. Technology Stack

| Layer | Technology | Justification |
|-------|-----------|---------------|
| Frontend | React / React Native | Web + mobile app for alert management |
| API Gateway | Kong / NGINX | Rate limiting, auth, routing |
| Backend Services | Java Spring Boot | High-performance stream processing, mature Kafka integration |
| Stream Processing | Kafka Streams | Stateful rule evaluation co-located with partitions |
| Message Broker | Apache Kafka | High-throughput event streaming, durable log |
| Primary Database | PostgreSQL | Alert rules, user profiles, alert history — ACID-compliant |
| Cache | Redis | Latest price cache, session store, cooldown tracking |
| Notification Providers | SMTP / Twilio / FCM+APNs | Multi-channel delivery (email, SMS, push) |
| Monitoring | Prometheus + Grafana | Latency metrics, alert pipeline health |
| CI/CD | GitHub Actions | Automated testing and deployment |

---

## 3. Module Breakdown

### 3.1 Market Data Ingestion
- Connect to market data feeds (Bloomberg, Reuters, exchange APIs)
- Normalize heterogeneous price formats into canonical schema
- Publish `PriceUpdated` events to Kafka partitioned by symbol
- Maintain Redis cache with latest prices per symbol
- Handle market open/close events and trading halts

### 3.2 Alert Rule Engine
- Consume `PriceUpdated` events from Kafka (partitioned by symbol)
- Maintain in-memory index of active rules per symbol
- Evaluate conditions (threshold, percentage change, crossing)
- Publish `AlertTriggered` events for matched rules
- Track cooldown state to prevent duplicate triggers
- Sync rule changes via CDC from PostgreSQL

### 3.3 Alert Management API
- CRUD operations for customer alert rules
- Alert rule validation (valid symbol, supported condition types)
- Alert history and delivery status queries
- User preference management (notification channels, quiet hours)
- Batch operations (enable/disable all alerts)

### 3.4 Notification Service
- Consume `AlertTriggered` events from Kafka
- Route to appropriate channel(s) based on user preferences
- Dispatch via email (SMTP), SMS (Twilio), push (FCM/APNs)
- Track delivery status and retry failures
- Respect quiet hours and user-level throttling

### 3.5 User Management
- Customer registration and profile management
- Notification channel verification (confirm email, verify phone)
- Preferences: default channels, quiet hours, language
- Subscription/plan enforcement (max alerts per tier)

---

## 4. Diagram Inventory

| # | Diagram | Format | File |
|---|---------|--------|------|
| 1 | C4 System Context | Mermaid | `c4-context.md` |
| 2 | C4 Container | Mermaid | `c4-container.md` |
| 3 | C4 Component — Market Data Ingestion | Mermaid | `c4-component-market-data.md` |
| 4 | C4 Component — Alert Rule Engine | Mermaid | `c4-component-rule-engine.md` |
| 5 | C4 Component — Notification Service | Mermaid | `c4-component-notification.md` |
| 6 | Integration Patterns | Mermaid | `integration-patterns.md` |
| 7 | Security Architecture Overlay | Mermaid | `security-architecture.md` |

---

## 5. Execution Order

1. **Phase 1 — Context** → `c4-context.md` (system boundaries, actors, external feeds/gateways)
2. **Phase 2 — Containers** → `c4-container.md` (services, databases, message broker, cache)
3. **Phase 3 — Components** → Component-level diagrams for Market Data, Rule Engine, Notification
4. **Phase 4 — Cross-cutting** → Integration patterns + security overlay
5. **Phase 5 — Verification** → Review consistency across diagrams, validate naming, confirm scalability constraints
