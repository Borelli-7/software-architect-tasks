# C4 Component Diagram — Alert Rule Engine

## Description
Internal components of the Alert Rule Engine service, showing how price events are consumed, rules are matched, conditions evaluated, and alert triggers published. This is the core intelligence of the system — designed for high throughput and low latency.

```mermaid
C4Component
    title Component Diagram — Alert Rule Engine

    Container_Boundary(ruleEngine, "Alert Rule Engine") {
        Component(priceConsumer, "Price Event Consumer", "Kafka Streams", "Consumes PriceUpdated events from Kafka partitions; each instance handles a subset of symbols")
        Component(ruleIndex, "Rule Index", "In-Memory HashMap", "Symbol-partitioned index of active rules — O(1) lookup by symbol; refreshed via CDC stream")
        Component(conditionEval, "Condition Evaluator", "Business Logic", "Evaluates rule conditions: threshold crossing, percentage change, absolute value comparison")
        Component(cooldownManager, "Cooldown Manager", "Redis Client", "Checks and sets cooldown state per rule to prevent duplicate triggers within configured window")
        Component(triggerPublisher, "Alert Trigger Publisher", "Kafka Producer", "Publishes AlertTriggered events for rules that matched and passed cooldown check")
        Component(ruleSyncConsumer, "Rule Sync Consumer", "CDC Consumer", "Subscribes to alert_rules DB changes via Debezium CDC; updates in-memory Rule Index in real-time")
        Component(marketStateHandler, "Market State Handler", "Event Handler", "Processes MarketOpened/Closed/Halted events to activate/deactivate rule evaluation per symbol")
    }

    Container(broker, "Message Broker", "Kafka", "Event streaming")
    ContainerDb(alertDb, "Alert Rules DB", "PostgreSQL", "Alert definitions")
    Container(priceCache, "Price Cache", "Redis", "Cooldown state")

    Rel(broker, priceConsumer, "PriceUpdated events", "Kafka/TLS")
    Rel(broker, ruleSyncConsumer, "CDC change events", "Kafka/TLS")
    Rel(broker, marketStateHandler, "MarketStateChanged events", "Kafka/TLS")

    Rel(priceConsumer, ruleIndex, "Lookup rules for symbol")
    Rel(ruleIndex, conditionEval, "Matching rules for evaluation")
    Rel(priceConsumer, conditionEval, "Current price data")
    Rel(conditionEval, cooldownManager, "Check cooldown for triggered rules")
    Rel(cooldownManager, priceCache, "GET/SET cooldown:{ruleId}", "Redis/TLS")
    Rel(cooldownManager, triggerPublisher, "Rules that passed cooldown")
    Rel(triggerPublisher, broker, "AlertTriggered events", "Kafka/TLS")
    Rel(ruleSyncConsumer, ruleIndex, "Insert/Update/Delete rules")
```

## Rule Evaluation Flow

```mermaid
stateDiagram-v2
    [*] --> PriceReceived: PriceUpdated event consumed
    PriceReceived --> RuleLookup: Extract symbol from event
    RuleLookup --> NoRules: No active rules for symbol
    RuleLookup --> EvaluateConditions: 1..N rules found
    NoRules --> [*]: Discard (no work)
    
    EvaluateConditions --> ConditionNotMet: Price does not satisfy rule condition
    EvaluateConditions --> ConditionMet: Price satisfies rule condition
    ConditionNotMet --> [*]: No trigger
    
    ConditionMet --> CheckCooldown: Rule matched
    CheckCooldown --> InCooldown: Last triggered within cooldown window
    CheckCooldown --> PublishTrigger: Cooldown expired or first trigger
    InCooldown --> [*]: Suppress duplicate
    PublishTrigger --> SetCooldown: Mark cooldown start
    SetCooldown --> [*]: AlertTriggered published
```

## Supported Condition Types

| Condition | Expression | Example |
|-----------|-----------|---------|
| **Price Above** | `price > threshold` | "Alert me if AAPL rises above €150" |
| **Price Below** | `price < threshold` | "Alert me if TSLA drops below €200" |
| **Percentage Change Up** | `changePercent > threshold` | "Alert me if MSFT gains more than 5% today" |
| **Percentage Change Down** | `changePercent < -threshold` | "Alert me if AMZN loses more than 3%" |
| **Price Crossing** | `prevPrice < target && price >= target` | "Alert me when GOOG crosses €140" |

## Scaling Strategy

| Aspect | Design |
|--------|--------|
| Partitioning | Kafka partitions keyed by symbol hash — each engine instance owns a partition subset |
| Rule capacity | ~100K–1M rules per instance (in-memory HashMap, ~200 bytes/rule) |
| Horizontal scale | Add Kafka consumer instances = linear throughput increase |
| Rebalancing | Kafka consumer group rebalance on scale-up; rule index rebuilds from CDC snapshot |
| Latency target | < 10ms from price event received to AlertTriggered published |
| Throughput | 50K–200K evaluations/second per instance |
