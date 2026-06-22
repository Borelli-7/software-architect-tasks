# Integration Patterns — Flight Status Distribution System

## Description
Defines how the Flight Status Distribution system modules communicate with each other and with external systems, using event-driven streaming, real-time push protocols, and synchronous API patterns.

## Event-Driven Architecture (Core Pipeline)

```mermaid
graph LR
    subgraph Producers
        ING[Flight Data Ingestion Service]
        PROC[Flight Status Processor]
    end

    subgraph Kafka["Apache Kafka — Event Streaming"]
        T1[flight.position.updated]
        T2[flight.status.changed]
        T3[flight.eta.revised]
        T4[flight.delayed]
        T5[feed.health]
        T6[delivery.status]
    end

    subgraph Consumers
        PROC2[Flight Status Processor]
        DIST[Distribution Service]
        HIST[History Writer]
        MON[Monitoring / Alerting]
    end

    ING -->|FlightPositionUpdated<br/>~1K-5K events/sec| T1
    ING -->|FlightStatusChanged<br/>departure/landing signals| T2
    ING -->|FeedHealthChanged| T5

    PROC -->|FlightStatusUpdated| T2
    PROC -->|FlightETARevised| T3
    PROC -->|FlightDelayed| T4

    T1 --> PROC2
    T2 --> PROC2
    T2 --> DIST
    T3 --> DIST
    T4 --> DIST

    T2 --> HIST
    T3 --> HIST
    T4 --> HIST

    T5 --> MON
    T6 --> MON

    DIST -->|DeliveryStatus| T6
```

## End-to-End Flight Status Flow (Sequence)

```mermaid
sequenceDiagram
    participant Radar as Central Radar/ATC
    participant ING as Ingestion Service
    participant Kafka as Kafka Broker
    participant PROC as Flight Processor
    participant Redis as Redis Cache
    participant DIST as Distribution Service
    participant DB as Consumer Registry DB
    participant Airline as Airline System (WebSocket)
    participant OTA as Travel Website (SSE)

    Radar->>ING: ASTERIX radar track (Flight AA123, pos: 48.8°N 2.3°E, alt: 3000ft)
    ING->>ING: Parse ASTERIX + correlate to flight plan
    ING->>Kafka: Publish FlightPositionUpdated (flightId=AA123, lat, lon, alt, speed)

    Kafka->>PROC: Consume FlightPositionUpdated (partition: hash(AA123))
    PROC->>PROC: Update state machine: AA123 → "approaching" (< 50nm from CDG)
    PROC->>PROC: Recalculate ETA: 14:32 UTC (was 14:45)
    PROC->>Kafka: Publish FlightStatusUpdated (AA123, status=approaching)
    PROC->>Kafka: Publish FlightETARevised (AA123, eta=14:32, previous=14:45)
    PROC->>Redis: SET flight:AA123 {status: approaching, eta: 14:32}

    Kafka->>DIST: Consume FlightStatusUpdated + FlightETARevised
    DIST->>DIST: Match subscriptions: Airline AA (by airline), OTA (by route JFK→CDG)
    DIST->>DB: Verify consumer tier + active subscriptions
    
    par Push to Airline
        DIST->>Airline: WebSocket frame: {flight: AA123, status: approaching, eta: 14:32}
        Airline-->>DIST: ACK
    and Push to OTA
        DIST->>OTA: SSE event: data: {flight: AA123, status: approaching, eta: 14:32}
    end

    DIST->>Kafka: Publish DeliveryStatus (consumers: [airline-AA, ota-travel], delivered: true)
```

## Synchronous API Calls

```mermaid
sequenceDiagram
    participant Hotel as Hotel Booking Platform
    participant GW as API Gateway
    participant API as Consumer REST API
    participant Cache as Redis Cache
    participant DB as Flight Status DB
    participant RegDB as Consumer Registry DB

    Note over Hotel: Hotel checks arrival time for guest's flight
    Hotel->>GW: GET /flights/AA123/status + X-API-Key: hotel-key-xyz
    GW->>GW: Validate API key, check rate limit (Standard tier: 100 req/min)
    GW->>API: Forward request + consumer context
    API->>Cache: GET flight:AA123
    Cache-->>API: {status: approaching, eta: 14:32, gate: B42, terminal: 2}
    API-->>GW: 200 OK {flightId: AA123, status: approaching, eta: "14:32Z", gate: "B42"}
    GW-->>Hotel: 200 OK

    Note over Hotel: Hotel queries all arrivals in next 2 hours
    Hotel->>GW: GET /flights?airport=CDG&type=arrivals&window=2h
    GW->>API: Forward
    API->>DB: SELECT flights WHERE destination='CDG' AND eta BETWEEN now AND now+2h
    DB-->>API: [{AA123, approaching, 14:32}, {LH456, en-route, 15:10}, ...]
    API-->>GW: 200 OK [{flights}]
    GW-->>Hotel: 200 OK
```

## External System Integration

```mermaid
graph TB
    subgraph FlightSystem["Flight Status Distribution System Boundary"]
        ING[Ingestion Service]
        PROC[Flight Processor]
        DIST[Distribution Service]
        API[Consumer REST API]
    end

    subgraph Inbound["Inbound Data Sources"]
        ATC[Central Radar/ATC System<br/>ASTERIX/TCP + ADS-B]
        WEATHER[Weather Service<br/>REST/HTTPS — METAR, TAF]
    end

    subgraph Outbound["Outbound Consumers"]
        AA[American Airlines<br/>WebSocket — full fleet]
        LH[Lufthansa<br/>WebSocket — full fleet]
        EXPEDIA[Expedia<br/>SSE — popular routes]
        BOOKING[Booking.com<br/>REST — arrival queries]
        MARRIOTT[Marriott Hotels<br/>REST — guest flight arrivals]
        FLIGHTRADAR[FlightRadar App<br/>WebSocket — all flights]
    end

    ATC -->|ASTERIX Cat062<br/>Radar surveillance| ING
    ATC -->|ADS-B messages<br/>Transponder positions| ING
    WEATHER -->|METAR/TAF<br/>Wind, visibility| PROC

    DIST -->|WebSocket/TLS<br/>Real-time push| AA
    DIST -->|WebSocket/TLS<br/>Real-time push| LH
    DIST -->|SSE/HTTPS<br/>Event stream| EXPEDIA
    DIST -->|WebSocket/TLS<br/>Full feed| FLIGHTRADAR

    API -->|REST/HTTPS<br/>On-demand queries| BOOKING
    API -->|REST/HTTPS<br/>Arrival queries| MARRIOTT
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Pub/Sub (Kafka)** | Position updates → Processor → Distribution | Loose coupling, independent scaling, replay capability |
| **Fan-out** | One FlightStatusUpdated → multiple consumer subscriptions | Single event triggers delivery to all subscribers for that flight/airline/route |
| **Protocol Adapter** | ASTERIX/ADS-B → canonical events | Isolates proprietary protocol complexity from business logic |
| **WebSocket (bidirectional)** | Airline systems, flight tracking apps | Low-latency push with acknowledgment; supports consumer commands (subscribe/unsubscribe) |
| **SSE (server-sent events)** | Travel websites, OTAs | Simple HTTP-based push; works through proxies; no client→server messaging needed |
| **Request/Response (REST)** | Hotel platforms, batch consumers | On-demand queries for consumers that don't need real-time push |
| **Cache-Aside** | Redis for latest flight status | Sub-millisecond reads for REST API; avoids querying the full pipeline |
| **Circuit Breaker** | Distribution → consumer connections | If a consumer is unreachable, stop pushing to avoid resource exhaustion |
| **Replay on Reconnect** | Consumer reconnects after disconnect | Guaranteed delivery — consumer receives missed events from last ACK offset |
| **Competing Consumers** | Multiple Distribution Service instances | Horizontal scaling of push delivery workload |
| **Event Sourcing** | Flight status history | Complete audit trail of all state transitions for compliance and debugging |

## Data Flow Metrics

| Stage | Throughput | Latency (p99) |
|-------|-----------|---------------|
| Radar → Kafka (ingestion) | 1K–5K events/sec | < 100ms |
| Kafka → Flight status processed | 1K–5K events/sec | < 200ms |
| Status change → Distribution push | ~100–500 status changes/min | < 300ms |
| End-to-end (radar signal → consumer notified) | — | < 1 second |
| REST API response (cache hit) | 10K req/sec | < 20ms |
| REST API response (DB query) | 2K req/sec | < 100ms |
