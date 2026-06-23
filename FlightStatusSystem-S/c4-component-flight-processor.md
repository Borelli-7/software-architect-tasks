# C4 Component Diagram — Flight Status Processor

## Description
Internal components of the Flight Status Processor service, showing how raw flight position events are consumed, flights are tracked through their lifecycle state machine, ETAs are calculated, and enriched status updates are published for distribution.

```mermaid
C4Component
    title Component Diagram — Flight Status Processor

    Container_Boundary(flightProcessor, "Flight Status Processor") {
        Component(positionConsumer, "Position Event Consumer", "Kafka Streams", "Consumes FlightPositionUpdated events from Kafka partitions; each instance handles a subset of flights")
        Component(flightStateMachine, "Flight State Machine", "State Engine", "Tracks each flight through its lifecycle: scheduled → boarding → departed → en-route → approaching → landed → arrived at gate")
        Component(etaCalculator, "ETA Calculator", "Computation Engine", "Calculates estimated arrival time based on current position, ground speed, distance remaining, and wind data")
        Component(enrichmentService, "Enrichment Service", "Data Aggregator", "Combines flight position with schedule data (origin, destination, airline, gate, terminal) and weather conditions")
        Component(statusPublisher, "Status Event Publisher", "Kafka Producer", "Publishes FlightStatusUpdated and FlightETARevised events to Kafka for downstream distribution")
        Component(historyWriter, "History Writer", "DB Client", "Persists flight status transitions and position snapshots to PostgreSQL for audit and replay")
        Component(scheduleLoader, "Schedule Loader", "Cache-Aside Client", "Loads and caches flight schedule/reference data from Schedule Service; refreshes on gate/terminal changes")
    }

    Container(broker, "Message Broker", "Kafka", "Event streaming")
    ContainerDb(flightDb, "Flight Status DB", "PostgreSQL", "Flight states, history, schedules")
    Container(statusCache, "Status Cache", "Redis", "Latest status per flight")
    System_Ext(weatherService, "Weather Service", "Met data for ETA")

    Rel(broker, positionConsumer, "FlightPositionUpdated, FlightStatusChanged", "Kafka/TLS")
    Rel(positionConsumer, flightStateMachine, "Position + status events")
    Rel(flightStateMachine, etaCalculator, "Flight in en-route/approaching state")
    Rel(etaCalculator, weatherService, "Fetch wind/weather data", "REST/HTTPS")
    Rel(flightStateMachine, enrichmentService, "State transitions")
    Rel(enrichmentService, scheduleLoader, "Request schedule data")
    Rel(scheduleLoader, flightDb, "Load flight schedule", "SQL/TLS")
    Rel(enrichmentService, statusPublisher, "Enriched status update")
    Rel(etaCalculator, statusPublisher, "Revised ETA")
    Rel(statusPublisher, broker, "FlightStatusUpdated, FlightETARevised", "Kafka/TLS")
    Rel(statusPublisher, statusCache, "SET flight:{id} latest status", "Redis/TLS")
    Rel(flightStateMachine, historyWriter, "State transition record")
    Rel(historyWriter, flightDb, "INSERT status history", "SQL/TLS")
```

## Flight Lifecycle State Machine

```mermaid
stateDiagram-v2
    [*] --> Scheduled: Flight plan filed
    Scheduled --> Boarding: Gate open + passengers boarding
    Boarding --> Departed: Wheels off / airborne signal
    Departed --> EnRoute: Climbing to cruise altitude
    EnRoute --> Approaching: Within terminal area (< 50nm from destination)
    Approaching --> Landed: Wheels on ground / touchdown signal
    Landed --> ArrivedAtGate: Parked at gate, engines off
    ArrivedAtGate --> [*]: Flight complete

    Scheduled --> Cancelled: Flight cancelled
    Boarding --> Cancelled: Flight cancelled after boarding
    Scheduled --> Delayed: Delay announced
    Delayed --> Boarding: Delay resolved, boarding begins
    Delayed --> Cancelled: Cancelled after delay
    EnRoute --> Diverted: Diversion to alternate airport
    Diverted --> [*]: Landed at alternate
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `FlightStatusUpdated` | Flight transitions to a new lifecycle state | Distribution Service, Consumer API |
| `FlightETARevised` | ETA changes by > 5 minutes from previous estimate | Distribution Service, Consumer API |
| `FlightDelayed` | Delay detected (schedule vs. actual departure) | Distribution Service, Airlines |
| `FlightDiverted` | Flight diverts to alternate airport | Distribution Service, Airlines, Airport Ops |
| `FlightCancelled` | Flight removed from active tracking | Distribution Service, All consumers |
| `GateChanged` | Arrival/departure gate reassigned | Distribution Service, Airlines, Passenger Apps |

## Scaling Strategy

| Aspect | Design |
|--------|--------|
| Partitioning | Kafka partitions keyed by flight ID — each processor instance owns a flight subset |
| Flight capacity | ~500–2,000 active flights per instance (state machine ~1KB/flight) |
| Horizontal scale | Add Kafka consumer instances = linear throughput increase |
| Rebalancing | Kafka consumer group rebalance on scale-up; state rebuilt from recent events |
| Latency target | < 200ms from position event received to FlightStatusUpdated published |
| ETA recalculation | Every 60 seconds for en-route flights, every 30 seconds for approaching flights |
