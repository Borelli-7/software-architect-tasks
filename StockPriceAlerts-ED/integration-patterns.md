# Integration Patterns — Stock Price Alerts

## Description
Defines how the Stock Price Alert system modules communicate with each other and with external systems, using event-driven streaming and synchronous API patterns.

## Event-Driven Architecture (Core Pipeline)

```mermaid
graph LR
    subgraph Producers
        MDS[Market Data Ingestion Service]
        API[Alert Management API]
    end

    subgraph Kafka["Apache Kafka — Event Streaming"]
        T1[stock.prices]
        T2[alert.triggers]
        T3[notification.status]
        T4[market.state]
        T5[alert.rules.cdc]
    end

    subgraph Consumers
        RE[Alert Rule Engine]
        NS[Notification Service]
        HIST[Alert History Writer]
        MON[Monitoring/Analytics]
    end

    MDS -->|PriceUpdated<br/>~200K events/sec| T1
    MDS -->|MarketOpened<br/>MarketClosed<br/>TradingHalted| T4
    API -->|RuleCreated/Updated/Deleted<br/>via Debezium CDC| T5

    T1 --> RE
    T4 --> RE
    T5 --> RE

    RE -->|AlertTriggered| T2

    T2 --> NS
    T2 --> HIST

    NS -->|NotificationSent<br/>NotificationFailed| T3

    T3 --> HIST
    T3 --> MON
```

## End-to-End Alert Flow (Sequence)

```mermaid
sequenceDiagram
    participant Feed as Market Data Feed
    participant MDS as Market Data Service
    participant Kafka as Kafka Broker
    participant RE as Rule Engine
    participant Redis as Redis Cache
    participant NS as Notification Service
    participant DB as User Preferences DB
    participant Email as Email Gateway
    participant SMS as SMS Gateway
    participant Push as Push Service

    Feed->>MDS: Price tick (AAPL = €152.30)
    MDS->>MDS: Normalize + validate
    MDS->>Kafka: Publish PriceUpdated (symbol=AAPL, price=152.30)
    MDS->>Redis: SET price:AAPL 152.30

    Kafka->>RE: Consume PriceUpdated (partition: hash(AAPL))
    RE->>RE: Lookup rules for AAPL (in-memory index)
    RE->>RE: Evaluate: "AAPL > €150" → MATCH
    RE->>Redis: GET cooldown:rule-123
    Redis-->>RE: null (no active cooldown)
    RE->>Redis: SET cooldown:rule-123 TTL=3600s
    RE->>Kafka: Publish AlertTriggered (ruleId=123, userId=456, symbol=AAPL, price=152.30)

    Kafka->>NS: Consume AlertTriggered
    NS->>DB: GET preferences for userId=456
    DB-->>NS: channels=[email, push], quietHours=null
    NS->>NS: Render templates (email + push)
    
    par Email delivery
        NS->>Email: Send alert email
        Email-->>NS: 202 Accepted
    and Push delivery
        NS->>Push: Send push notification
        Push-->>NS: 200 OK
    end
    
    NS->>Kafka: Publish NotificationSent (channels=[email,push])
```

## Synchronous API Calls

```mermaid
sequenceDiagram
    participant App as Web/Mobile App
    participant GW as API Gateway
    participant API as Alert Management API
    participant DB as Alert Rules DB
    participant Cache as Redis Cache
    participant UserSvc as User Service
    participant UserDB as User DB

    Note over App: Customer creates a new alert
    App->>GW: POST /alerts {symbol: "AAPL", condition: "above", threshold: 150}
    GW->>GW: Validate JWT, check rate limit
    GW->>API: Forward request + user context
    API->>Cache: GET price:AAPL (validate symbol exists)
    Cache-->>API: 148.50 (symbol valid, current price)
    API->>DB: INSERT alert rule
    DB-->>API: rule created (id=123)
    API-->>GW: 201 Created {id: 123, status: "active"}
    GW-->>App: 201 Created

    Note over App: Customer views alert history
    App->>GW: GET /alerts/123/history
    GW->>API: Forward
    API->>DB: SELECT triggered_alerts WHERE rule_id=123
    DB-->>API: [{triggeredAt, price, notified}]
    API-->>GW: 200 OK [history entries]
    GW-->>App: 200 OK
```

## External System Integration

```mermaid
graph TB
    subgraph AlertSystem["Stock Price Alert System Boundary"]
        MDS[Market Data Service]
        NS[Notification Service]
        API[Alert Management API]
        UserSvc[User Service]
    end

    subgraph External["External Systems"]
        BLOOM[Bloomberg Feed]
        REUT[Reuters Feed]
        EXCH[Exchange Direct Feeds]
        SMTP[Email Gateway<br/>SMTP]
        TWILIO[SMS Gateway<br/>Twilio REST API]
        FCM[Firebase Cloud Messaging]
        APNS[Apple Push Notification]
        BROKER[Brokerage Platform<br/>Customer accounts]
    end

    BLOOM -->|WebSocket<br/>Real-time prices| MDS
    REUT -->|FIX protocol<br/>Real-time prices| MDS
    EXCH -->|WebSocket<br/>L1 quotes| MDS

    NS -->|SMTP/TLS<br/>Alert emails| SMTP
    NS -->|HTTPS/REST<br/>SMS messages| TWILIO
    NS -->|HTTPS/REST<br/>Android push| FCM
    NS -->|HTTPS/HTTP2<br/>iOS push| APNS

    API -->|REST/gRPC<br/>Validate customer tier| BROKER
    UserSvc -->|REST<br/>Sync customer profiles| BROKER
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Pub/Sub (Kafka)** | Price updates → Rule Engine → Notifications | Loose coupling, independent scaling, replay capability |
| **CDC (Debezium)** | Alert rules DB → Rule Engine in-memory cache | Real-time sync without polling; no dual-write complexity |
| **Fan-out** | One PriceUpdated → multiple rule evaluations | Single price event triggers evaluation of all rules for that symbol |
| **Circuit Breaker** | Notification → external gateways (email/SMS/push) | Fault tolerance when providers are degraded |
| **Retry + DLQ** | Failed notification delivery | Guaranteed delivery with error isolation |
| **Idempotent Consumer** | Notification deduplication | At-least-once Kafka delivery may produce duplicates |
| **Competing Consumers** | Multiple Rule Engine instances per partition group | Horizontal scaling of rule evaluation |
| **Event Sourcing** | Alert trigger history | Complete audit trail of all triggers and deliveries |
| **API Gateway** | All client-to-service calls | Centralized auth, rate limiting, routing |
| **Saga (Choreography)** | Price → Evaluate → Notify → Track | No central orchestrator; each service reacts to events |

## Data Flow Metrics

| Stage | Throughput | Latency (p99) |
|-------|-----------|---------------|
| Feed → Kafka | 200K events/sec | < 50ms |
| Kafka → Rule evaluation | 200K evaluations/sec | < 10ms |
| Rule match → AlertTriggered | ~1K–10K triggers/sec (market dependent) | < 5ms |
| AlertTriggered → Notification sent | ~1K–10K/sec | < 500ms (email), < 200ms (push) |
| End-to-end (price change → customer notified) | — | < 2 seconds |
