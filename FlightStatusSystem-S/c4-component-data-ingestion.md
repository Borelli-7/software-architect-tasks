# C4 Component Diagram — Flight Data Ingestion Service

## Description
Internal components of the Flight Data Ingestion microservice, showing protocol adapters, data normalizers, validators, and publishers that transform raw radar/transponder data into canonical flight events.

```mermaid
C4Component
    title Component Diagram — Flight Data Ingestion Service

    Container_Boundary(ingestionSvc, "Flight Data Ingestion Service") {
        Component(asterixAdapter, "ASTERIX Protocol Adapter", "TCP Client / Binary Parser", "Maintains persistent connection to central radar system; decodes ASTERIX Category 062 surveillance data")
        Component(adsbReceiver, "ADS-B Receiver", "UDP/TCP Listener", "Receives Automatic Dependent Surveillance-Broadcast messages from aircraft transponders via the central system")
        Component(dataNormalizer, "Data Normalizer", "Transformer", "Converts heterogeneous radar/transponder formats into canonical FlightPosition schema with unified identifiers")
        Component(positionValidator, "Position Validator", "Validation Layer", "Detects anomalies: impossible coordinates, stale timestamps, duplicate tracks; filters corrupt data")
        Component(eventPublisher, "Event Publisher", "Kafka Producer", "Publishes validated FlightPositionUpdated and FlightStatusChanged events to Kafka, partitioned by flight ID")
        Component(heartbeatMonitor, "Heartbeat Monitor", "Health Check", "Monitors central system liveness via periodic heartbeats; triggers FeedHealthChanged on outage or recovery")
        Component(flightCorrelator, "Flight Correlator", "Correlation Engine", "Matches radar tracks to flight plans using callsign, squawk code, and position — resolves track identity")
    }

    System_Ext(centralATC, "Central Radar/ATC System", "Airport control area — radar and transponder feeds")
    Container(broker, "Message Broker", "Kafka", "Event streaming")
    Container(statusCache, "Status Cache", "Redis", "Feed health state")

    Rel(centralATC, asterixAdapter, "Streams ASTERIX radar data", "TCP/mTLS")
    Rel(centralATC, adsbReceiver, "Streams ADS-B messages", "TCP/mTLS")
    Rel(asterixAdapter, dataNormalizer, "Raw radar tracks")
    Rel(adsbReceiver, dataNormalizer, "Raw ADS-B positions")
    Rel(dataNormalizer, flightCorrelator, "Normalized positions")
    Rel(flightCorrelator, positionValidator, "Correlated flight positions")
    Rel(positionValidator, eventPublisher, "Validated flight events")
    Rel(eventPublisher, broker, "FlightPositionUpdated, FlightStatusChanged", "Kafka/TLS")
    Rel(heartbeatMonitor, centralATC, "Heartbeat probe", "TCP")
    Rel(heartbeatMonitor, broker, "FeedHealthChanged events", "Kafka/TLS")
    Rel(heartbeatMonitor, statusCache, "SET feed:health status", "Redis/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `FlightPositionUpdated` | Valid position update received for a tracked flight | Flight Status Processor |
| `FlightStatusChanged` | Major status transition detected (departed, landed, etc.) | Flight Status Processor, Distribution Service |
| `FeedHealthChanged` | Central system connection lost or restored | Airport Ops Dashboard, Monitoring |
| `TrackIdentified` | Radar track successfully correlated to a flight plan | Flight Status Processor |
| `TrackLost` | Radar track disappeared without expected landing | Flight Status Processor, Airport Ops |
| `DuplicateTrackDetected` | Multiple tracks appear for same flight — data quality issue | Airport Ops (monitoring alert) |

## Data Flow Characteristics

| Metric | Value |
|--------|-------|
| Inbound throughput | ~1,000–5,000 position updates/second (peak hours) |
| Flights tracked simultaneously | 200–500 (depending on airport size) |
| Kafka partitions | 32–64 (partitioned by flight ID hash) |
| Latency target | < 100ms from radar signal to Kafka publish |
| Availability | 99.99% — aviation-grade reliability requirement |
| Feed failover | Active-passive per ATC connection; < 3s switchover |
| Data validation reject rate | < 0.1% (corrupt/stale positions filtered) |
