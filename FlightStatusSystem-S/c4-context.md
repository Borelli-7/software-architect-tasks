# C4 System Context Diagram — Flight Status Distribution System

## Description
Shows the Flight Status Distribution system in context with its external actors, the central airport radar/ATC system, and all external consumer systems that receive flight data.

```mermaid
C4Context
    title System Context Diagram — Flight Status Distribution

    Person(airportOps, "Airport Operations Staff", "Monitors system health, manages consumer access, handles data quality issues")
    Person(airlineOps, "Airline Operations Center", "Tracks their airline's flights, receives real-time status updates for crew scheduling and passenger comms")

    System(flightSystem, "Flight Status Distribution System", "Ingests real-time flight data from the airport's central radar/ATC system and distributes status updates to authorized external consumers — airlines, travel sites, hotel platforms")

    System_Ext(centralATC, "Central Radar/ATC System", "Airport control area real-time system — receives radar tracks, ADS-B transponder data, and flight progress updates")
    System_Ext(weatherService, "Weather Data Service", "Provides meteorological data for ETA calculations and delay predictions")

    System_Ext(airlines, "Airline Systems", "Flight operations, crew management, and passenger notification systems of individual airlines")
    System_Ext(travelSites, "Travel Websites / OTAs", "Online travel agencies displaying real-time flight status to travelers")
    System_Ext(hotelSites, "Hotel Booking Platforms", "Hotel systems adjusting check-in/shuttle services based on flight arrivals")
    System_Ext(passengerApps, "Passenger Mobile Apps", "Third-party apps providing flight tracking to individual travelers")

    Rel(airportOps, flightSystem, "Manages consumers, monitors health")
    Rel(airlineOps, flightSystem, "Views operational dashboard, configures subscriptions")

    Rel(centralATC, flightSystem, "Streams radar tracks and flight progress", "ASTERIX/TCP, ADS-B")
    Rel(weatherService, flightSystem, "Provides weather updates", "REST/HTTPS")

    Rel(flightSystem, airlines, "Pushes flight status updates", "WebSocket/HTTPS")
    Rel(flightSystem, travelSites, "Pushes/serves flight status", "WebSocket/SSE/REST")
    Rel(flightSystem, hotelSites, "Serves arrival status on-demand", "REST/HTTPS")
    Rel(flightSystem, passengerApps, "Pushes flight tracking data", "WebSocket/SSE")
```

## Key Observations

- **2 internal actor types**: Airport Operations Staff (system administrators) and Airline Operations Centers (power users with operational dashboards)
- **1 primary inbound data source**: Central Radar/ATC System providing real-time flight tracking data via proprietary protocols (ASTERIX, ADS-B)
- **1 secondary inbound source**: Weather Data Service for ETA enrichment
- **4 outbound consumer categories**: Airlines (operational), Travel Websites (informational), Hotel Platforms (logistics), Passenger Apps (personal tracking)
- **Dual distribution model**: Push (WebSocket/SSE) for real-time consumers, Pull (REST) for on-demand consumers
- The central ATC system is in a restricted network zone — the connection is highly secured (mTLS, IP whitelist)
- Consumer diversity requires flexible filtering — each consumer type needs different subsets of data
