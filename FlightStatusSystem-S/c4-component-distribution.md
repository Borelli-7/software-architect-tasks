# C4 Component Diagram — Distribution Service

## Description
Internal components of the Distribution Service, showing how flight status updates are consumed from Kafka, matched to consumer subscriptions, and pushed in real-time via WebSocket/SSE connections to authorized external consumers.

```mermaid
C4Component
    title Component Diagram — Distribution Service

    Container_Boundary(distributionSvc, "Distribution Service") {
        Component(statusConsumer, "Status Event Consumer", "Kafka Consumer", "Consumes FlightStatusUpdated, FlightETARevised, and FlightDelayed events from Kafka")
        Component(subscriptionMatcher, "Subscription Matcher", "Routing Logic", "Matches incoming events against active consumer subscriptions — filters by airline, route, airport, or specific flight")
        Component(connectionManager, "Connection Manager", "WebSocket/SSE Server", "Manages persistent connections: handshake, authentication, heartbeat, graceful disconnect, reconnection with replay")
        Component(pushEngine, "Push Engine", "Event Dispatcher", "Formats and delivers matched events to connected consumers via their active WebSocket/SSE channels")
        Component(rateLimiter, "Rate Limiter", "Token Bucket", "Enforces per-consumer rate limits based on subscription tier; queues excess updates for batch delivery")
        Component(deliveryTracker, "Delivery Tracker", "Event Logger", "Records push delivery status (sent, acknowledged, failed); enables replay on consumer reconnection")
        Component(subscriptionStore, "Subscription Store", "Redis Client", "Maintains in-memory subscription index from Redis; hot-reloads on consumer subscription changes")
    }

    Container(broker, "Message Broker", "Kafka", "Event streaming")
    ContainerDb(consumerDb, "Consumer Registry DB", "PostgreSQL", "Consumer subscriptions")
    Container(statusCache, "Status Cache", "Redis", "Subscription indexes, connection state")
    System_Ext(airlines, "Airline Systems", "Flight ops consumers")
    System_Ext(travelSites, "Travel Websites", "OTA consumers")
    System_Ext(hotelSites, "Hotel Platforms", "Arrival consumers")
    System_Ext(passengerApps, "Passenger Apps", "Flight tracking consumers")

    Rel(broker, statusConsumer, "FlightStatusUpdated, FlightETARevised events", "Kafka/TLS")
    Rel(statusConsumer, subscriptionMatcher, "Flight status event payload")
    Rel(subscriptionMatcher, subscriptionStore, "Lookup matching subscriptions")
    Rel(subscriptionStore, statusCache, "GET subscriptions by filter", "Redis/TLS")
    Rel(subscriptionMatcher, pushEngine, "Matched events + target consumers")
    Rel(pushEngine, rateLimiter, "Check consumer rate budget")
    Rel(rateLimiter, pushEngine, "Approved / queued")

    Rel(pushEngine, connectionManager, "Dispatch to consumer connection")
    Rel(connectionManager, airlines, "Push flight update", "WebSocket/TLS")
    Rel(connectionManager, travelSites, "Push flight update", "SSE/HTTPS")
    Rel(connectionManager, hotelSites, "Push flight update", "SSE/HTTPS")
    Rel(connectionManager, passengerApps, "Push flight update", "WebSocket/TLS")

    Rel(connectionManager, deliveryTracker, "Delivery result (ack/fail)")
    Rel(deliveryTracker, statusCache, "SET delivery:{consumerId}:{eventId}", "Redis/TLS")
    Rel(deliveryTracker, broker, "DeliveryStatus events", "Kafka/TLS")
```

## Subscription Model

| Filter Type | Example | Use Case |
|-------------|---------|----------|
| **By Airline** | `airline=AA` | American Airlines receives all their own flights |
| **By Route** | `origin=JFK&destination=LAX` | OTA showing JFK→LAX flights |
| **By Airport** | `airport=CDG&type=arrivals` | Hotel shuttle service at CDG |
| **By Flight** | `flightId=AA123` | Passenger tracking a specific flight |
| **By Region** | `region=europe` | Travel site covering European routes |
| **All Flights** | `filter=*` | Premium partners needing full airport feed |

## Delivery Guarantees

| Aspect | Design |
|--------|--------|
| **Semantics** | At-least-once delivery — consumers deduplicate by event sequence number |
| **Reconnection replay** | On reconnect, consumer receives missed events from last acknowledged offset (stored in Redis) |
| **Heartbeat** | Server sends ping every 30s; client must respond within 10s or connection is considered dead |
| **Backpressure** | If consumer falls behind, events are buffered up to 1000; beyond that, oldest events are dropped with a gap notification |
| **Rate limiting** | Per-consumer token bucket; excess events queued and delivered in next available window |
| **Circuit breaker** | If consumer fails to acknowledge 10 consecutive events, connection is suspended and admin alerted |
| **Graceful shutdown** | On server restart, consumers receive close frame with reconnect hint; replay from last ack |

## Consumer Tier Capabilities

| Tier | Max Connections | Push Rate | Filters | Replay Window |
|------|----------------|-----------|---------|---------------|
| **Basic** | 2 | 10 events/sec | By flight only | 5 minutes |
| **Standard** | 10 | 100 events/sec | By airline, route, airport | 30 minutes |
| **Premium** | Unlimited | 1,000 events/sec | All filters + wildcard | 24 hours |
| **Partner** | Unlimited | Unlimited | Full feed | 48 hours |

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `ConsumerConnected` | New WebSocket/SSE connection established | Monitoring, Admin Dashboard |
| `ConsumerDisconnected` | Connection closed (graceful or timeout) | Monitoring, Admin Dashboard |
| `DeliveryFailed` | Push to consumer failed after retries | Admin alerting, audit log |
| `RateLimitExceeded` | Consumer exceeded their tier's push rate | Consumer notification, admin log |
