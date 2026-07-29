# C4 Component Diagram — Market Data Ingestion Service

## Description
Internal components of the Market Data Ingestion microservice, showing feed connectors, normalizers, and publishers that transform raw market data into canonical price events.

```mermaid
C4Component
    title Component Diagram — Market Data Ingestion Service

    Container_Boundary(marketDataSvc, "Market Data Ingestion Service") {
        Component(feedConnector, "Feed Connector", "WebSocket/FIX Client", "Maintains persistent connections to market data providers; handles reconnection and failover")
        Component(priceNormalizer, "Price Normalizer", "Transformer", "Converts heterogeneous feed formats into canonical PriceUpdate schema with unified symbol taxonomy")
        Component(priceValidator, "Price Validator", "Validation Layer", "Detects anomalies: stale data, impossible prices, trading halts; filters bad ticks")
        Component(pricePublisher, "Price Publisher", "Kafka Producer", "Publishes validated PriceUpdated events to Kafka, partitioned by stock symbol")
        Component(cacheWriter, "Cache Writer", "Redis Client", "Writes latest price per symbol to Redis for on-demand API queries")
        Component(marketStateTracker, "Market State Tracker", "State Machine", "Tracks market open/close, trading halts per exchange; emits MarketStateChanged events")
    }

    System_Ext(marketData, "Market Data Providers", "Bloomberg, Reuters, exchange feeds")
    Container(broker, "Message Broker", "Kafka", "Event streaming")
    Container(priceCache, "Price Cache", "Redis", "Latest prices")

    Rel(marketData, feedConnector, "Streams raw prices", "WebSocket/FIX")
    Rel(feedConnector, priceNormalizer, "Raw price ticks")
    Rel(priceNormalizer, priceValidator, "Normalized prices")
    Rel(priceValidator, pricePublisher, "Validated prices")
    Rel(priceValidator, marketStateTracker, "Trading halt signals")
    Rel(pricePublisher, broker, "PriceUpdated events", "Kafka/TLS")
    Rel(marketStateTracker, broker, "MarketStateChanged events", "Kafka/TLS")
    Rel(priceValidator, cacheWriter, "Latest valid prices")
    Rel(cacheWriter, priceCache, "SET price:{symbol}", "Redis/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `PriceUpdated` | Valid price tick received for a symbol | Alert Rule Engine, Price Cache |
| `MarketOpened` | Exchange trading session begins | Alert Rule Engine (activate rules) |
| `MarketClosed` | Exchange trading session ends | Alert Rule Engine (deactivate rules) |
| `TradingHalted` | Individual stock trading halt detected | Alert Rule Engine (pause rules for symbol) |
| `TradingResumed` | Halt lifted for a stock | Alert Rule Engine (resume rules for symbol) |
| `FeedDisconnected` | Connection lost to a data provider | System Admin (monitoring alert) |
| `FeedReconnected` | Connection re-established after outage | System Admin (monitoring resolved) |

## Data Flow Characteristics

| Metric | Value |
|--------|-------|
| Inbound throughput | ~50,000–200,000 ticks/second (market hours) |
| Symbols tracked | 5,000–10,000 across multiple exchanges |
| Kafka partitions | 64–128 (partitioned by symbol hash) |
| Latency target | < 50ms from feed to Kafka publish |
| Availability | 99.95% during market hours |
| Feed failover | Active-passive per provider; < 5s switchover |
