# Flight Status Distribution System Architecture Plan — Airport Authority

## Executive Summary

Real-time flight status distribution system for a major airport authority, providing live flight information to external consumers including airlines, travel websites, hotel booking platforms, and passenger-facing applications. The architecture follows an event-driven, stream-processing approach to ingest flight data from the airport's central radar/ATC system and distribute status updates to thousands of concurrent subscribers. Supports hundreds of simultaneous flights, multiple consumer tiers, and both push (WebSocket/SSE) and pull (REST) distribution models.

---

## 1. Architectural Decisions

### 1.1 Real-Time Ingestion Strategy
| Decision | Rationale |
|----------|-----------|
| **Event-driven ingestion from central ATC system** | Subscribe to radar/transponder events for sub-second position updates; avoids wasteful polling of the control system |
| **Protocol adapter pattern** | Central system uses proprietary protocols (ASTERIX, ADS-B); adapters normalize into canonical events |
| **Partitioning by flight ID** | Each flight's updates are sequentially ordered within a single partition — ensures consistency |
| **Heartbeat monitoring** | Detect central system outages within seconds; trigger failover or stale-data warnings |

### 1.2 Distribution Strategy
| Decision | Rationale |
|----------|-----------|
| **Pub/Sub for push distribution** | Consumers subscribe to topics (by airline, airport, route); updates pushed immediately on change |
| **WebSocket + SSE for real-time consumers** | Low-latency bidirectional/server-push for travel websites and airline ops dashboards |
| **REST API for on-demand queries** | Hotel booking sites and batch consumers poll current status without maintaining connections |
| **Consumer-specific data filtering** | Each consumer sees only relevant flights (their airline, their routes) — reduces noise and bandwidth |

### 1.3 Scalability Strategy
| Decision | Rationale |
|----------|-----------|
| **Horizontal scaling of distribution layer** | WebSocket connections are stateful; scale by adding gateway nodes with sticky sessions |
| **Redis cache for latest status** | On-demand REST queries served from cache — no load on processing pipeline |
| **Consumer rate limiting per tier** | Prevent any single consumer from overwhelming the system |
| **Event broker (Kafka) as backbone** | Decouples ingestion from distribution; each consumer group reads independently |

### 1.4 Reliability Strategy
| Decision | Rationale |
|----------|-----------|
| **At-least-once delivery + idempotency** | No flight status updates lost; consumers deduplicate by sequence number |
| **Graceful degradation** | If central system feed drops, serve last-known status with staleness indicator |
| **Multi-zone deployment** | Active-active across availability zones for the distribution layer |
| **Consumer reconnection protocol** | Clients receive missed events on reconnect via replay from offset |

### 1.5 Security Strategy
| Decision | Rationale |
|----------|-----------|
| **API key + OAuth 2.0 for consumers** | Identifies consumer, enforces tier limits, and audits access |
| **mTLS for central system connection** | Radar feed is sensitive; mutual authentication ensures only authorized systems connect |
| **TLS everywhere** | All data in transit encrypted |
| **Network segmentation** | Central system feed in restricted zone; consumer-facing APIs in DMZ |

---

## 2. Technology Stack

| Layer | Technology | Justification |
|-------|-----------|---------------|
| Ingestion Adapters | Go / Rust | Low-latency, high-throughput binary protocol parsing (ASTERIX/ADS-B) |
| Stream Processing | Java Spring Boot + Kafka Streams | Stateful flight tracking, mature Kafka integration |
| Message Broker | Apache Kafka | High-throughput event streaming, durable log, consumer group replay |
| Distribution Gateway | Node.js (WebSocket/SSE) | Efficient handling of thousands of concurrent persistent connections |
| REST API | Java Spring Boot | Standard REST endpoints for on-demand queries |
| Primary Database | PostgreSQL | Flight schedules, consumer registrations, subscription rules — ACID |
| Cache | Redis | Latest flight status per flight, subscription lookups |
| API Gateway | Kong / NGINX | Rate limiting, API key validation, SSL termination |
| Monitoring | Prometheus + Grafana | Pipeline latency, consumer connection health, feed status |
| CI/CD | GitHub Actions | Automated testing and deployment |

---

## 3. Module Breakdown

### 3.1 Flight Data Ingestion
- Connect to central airport radar/ATC system via proprietary protocols
- Parse ASTERIX radar data and ADS-B transponder messages
- Normalize into canonical `FlightPositionUpdated` and `FlightStatusChanged` events
- Publish events to Kafka partitioned by flight ID
- Monitor central system heartbeat; emit `FeedHealthChanged` on outage/recovery

### 3.2 Flight Status Processor
- Consume position and status events from Kafka
- Maintain flight state machine (scheduled → boarding → departed → en-route → approaching → landed → arrived)
- Calculate ETA based on position, speed, and weather data
- Enrich with schedule data (origin, destination, airline, gate)
- Publish `FlightStatusUpdated` and `FlightETARevised` events

### 3.3 Distribution Service
- Manage consumer subscriptions (by airline, route, airport, specific flight)
- Push updates via WebSocket/SSE to subscribed consumers
- Handle consumer connection lifecycle (connect, subscribe, heartbeat, reconnect)
- Enforce per-consumer rate limits and tier-based access
- Track delivery status and consumer acknowledgments

### 3.4 Consumer API Gateway
- REST endpoints for on-demand flight status queries
- Consumer registration and API key management
- Subscription management endpoints (CRUD)
- Rate limiting, authentication, request validation
- API documentation (OpenAPI/Swagger)

### 3.5 Flight Schedule Management
- Maintain flight schedule database (departures, arrivals, gates, terminals)
- Provide reference data for enrichment (airline codes, airport codes, routes)
- Handle schedule amendments and gate changes

---

## 4. Diagram Inventory

| # | Diagram | Format | File |
|---|---------|--------|------|
| 1 | C4 System Context | Mermaid | `c4-context.md` |
| 2 | C4 Container | Mermaid | `c4-container.md` |
| 3 | C4 Component — Flight Data Ingestion | Mermaid | `c4-component-data-ingestion.md` |
| 4 | C4 Component — Flight Status Processor | Mermaid | `c4-component-flight-processor.md` |
| 5 | C4 Component — Distribution Service | Mermaid | `c4-component-distribution.md` |
| 6 | Integration Patterns | Mermaid | `integration-patterns.md` |
| 7 | Security Architecture Overlay | Mermaid | `security-architecture.md` |

---

## 5. Execution Order

1. **Phase 1 — Context** → `c4-context.md` (system boundaries, actors, central system, external consumers)
2. **Phase 2 — Containers** → `c4-container.md` (services, databases, message broker, cache, gateways)
3. **Phase 3 — Components** → Component-level diagrams for Data Ingestion, Flight Processor, Distribution
4. **Phase 4 — Cross-cutting** → Integration patterns + security overlay
