# Integration Patterns — Telecom CRM Platform

## Description
Defines how the platform's modules communicate with each other and with external systems (BSS/OSS, communication channels, regulatory), using event-driven and synchronous patterns to support multi-category customer management and policy-driven action execution.

## Event-Driven Architecture (Inter-Module)

```mermaid
graph LR
    subgraph Producers
        CUSTOMER[Customer Management]
        CATEGORY[Category Engine]
        POLICY[Policy Engine]
        CAMPAIGN[Campaign & Action Service]
        ORDER[Order Management]
    end

    subgraph Kafka["Apache Kafka — Event Bus"]
        T1[customer.lifecycle]
        T2[category.membership]
        T3[policy.management]
        T4[campaign.execution]
        T5[order.lifecycle]
    end

    subgraph Consumers
        NOTIF[Notification Service]
        CHANNEL[Channel Orchestration]
        ANALYTICS[Analytics Service]
        CAMPAIGN_C[Campaign Service]
        CATEGORY_C[Category Engine]
        INTEGRATION[BSS/OSS Integration]
    end

    CUSTOMER -->|CustomerCreated<br/>CustomerUpdated<br/>LifecycleTransitioned<br/>ConsentChanged| T1
    CATEGORY -->|CategoryAssigned<br/>CategoryRemoved<br/>CategoryTransitioned<br/>ConflictResolved| T2
    POLICY -->|PolicyActivated<br/>PolicyDeprecated<br/>PolicyRolledBack<br/>ABTestCompleted| T3
    CAMPAIGN -->|CampaignLaunched<br/>OfferAccepted<br/>OfferExpired<br/>ActionExecuted| T4
    ORDER -->|OrderCreated<br/>OrderCompleted<br/>PortInCompleted<br/>PlanChanged| T5

    T1 --> CATEGORY_C
    T1 --> CAMPAIGN_C
    T1 --> ANALYTICS
    T1 --> NOTIF

    T2 --> CAMPAIGN_C
    T2 --> ANALYTICS
    T2 --> NOTIF

    T3 --> CAMPAIGN_C
    T3 --> ANALYTICS

    T4 --> ANALYTICS
    T4 --> NOTIF
    T4 --> CHANNEL

    T5 --> CATEGORY_C
    T5 --> ANALYTICS
    T5 --> NOTIF
    T5 --> INTEGRATION
```

## End-to-End: New Customer Onboarding Flow

```mermaid
sequenceDiagram
    participant Customer as Customer (Store/App)
    participant Portal as Agent Portal / App
    participant GW as API Gateway
    participant CustSvc as Customer Service
    participant CatEngine as Category Engine
    participant PolicyEngine as Policy Engine
    participant CampaignSvc as Campaign Service
    participant OrderSvc as Order Management
    participant BSS as BSS Integration
    participant ChannelSvc as Channel Orchestration
    participant Notif as Notification Service

    Customer->>Portal: Sign up (name, CPF, plan selection)
    Portal->>GW: POST /customers (identity + plan choice)
    GW->>GW: Validate token, resolve channel
    GW->>CustSvc: Create customer

    CustSvc->>CustSvc: Golden record check (dedup)
    CustSvc->>CustSvc: Create customer (Lead state)
    CustSvc-->>Portal: Customer created

    Note over CustSvc: Publishes CustomerCreated event
    CustSvc-)CatEngine: CustomerCreated event
    CatEngine->>CatEngine: Auto-assign categories (Individual + Prospect + Digital Channel)
    CatEngine-)PolicyEngine: CategoryAssigned event

    Portal->>GW: POST /orders (plan, SIM, payment)
    GW->>OrderSvc: Create order
    OrderSvc->>BSS: Provision plan + activate SIM
    BSS-->>OrderSvc: Activation confirmed
    OrderSvc->>CustSvc: Update lifecycle (Lead → Active)

    Note over CustSvc: Publishes LifecycleTransitioned event
    CustSvc-)CatEngine: LifecycleTransitioned event
    CatEngine->>CatEngine: Transition Prospect → New Customer + Active
    CatEngine-)PolicyEngine: CategoryTransitioned event

    PolicyEngine->>PolicyEngine: Evaluate policies for [Individual, New Customer, Active, Digital]
    PolicyEngine-)CampaignSvc: Applicable actions (welcome campaign, first-month bonus, referral offer)

    CampaignSvc->>CampaignSvc: Enroll in welcome campaign
    CampaignSvc->>ChannelSvc: Dispatch welcome communications
    ChannelSvc->>Notif: Send welcome SMS + email + push
    Notif-->>Customer: Welcome message + offer details
```

## Policy Change Propagation Flow

```mermaid
sequenceDiagram
    participant MA as Marketing Analyst
    participant Builder as Policy Builder UI
    participant PolicySvc as Policy Engine
    participant Simulation as Simulation Engine
    participant Senior as Senior Analyst
    participant CampaignSvc as Campaign Service
    participant Analytics as Analytics Service

    MA->>Builder: Create new policy (VIP + Business → exclusive 5G offer)
    Builder->>PolicySvc: Save policy (Draft)
    
    MA->>PolicySvc: Run simulation
    PolicySvc->>Simulation: Execute against historical data
    Simulation-->>PolicySvc: Results (12,000 customers eligible, projected 8% uptake)
    PolicySvc-->>MA: Simulation report

    MA->>PolicySvc: Submit for approval
    PolicySvc->>Senior: Notification — policy pending review
    Senior->>PolicySvc: Approve (no conflicts detected)
    
    PolicySvc->>PolicySvc: Compile and activate policy
    PolicySvc->>PolicySvc: Invalidate cache (blue-green swap)

    Note over PolicySvc: Publishes PolicyActivated event
    PolicySvc-)CampaignSvc: PolicyActivated event
    CampaignSvc->>CampaignSvc: Re-evaluate eligible customers
    CampaignSvc->>CampaignSvc: Enroll eligible [VIP + Business] customers
    
    PolicySvc-)Analytics: PolicyActivated event
    Analytics->>Analytics: Begin tracking policy performance metrics

    Note over CampaignSvc: No code deployment needed —<br/>system adapted to new marketing policy at runtime
```

## Category Transition Triggered Actions

```mermaid
sequenceDiagram
    participant Analytics as Data Analytics Platform
    participant CustSvc as Customer Service
    participant CatEngine as Category Engine
    participant PolicyEngine as Policy Engine
    participant CampaignSvc as Campaign Service
    participant ChannelSvc as Channel Orchestration

    Analytics->>CustSvc: Batch update — churn scores refreshed
    CustSvc->>CustSvc: Customer #12345 churn score = 0.82 (threshold: 0.7)
    CustSvc->>CustSvc: Transition lifecycle: Active → At-Risk

    Note over CustSvc: Publishes LifecycleTransitioned event
    CustSvc-)CatEngine: LifecycleTransitioned (Active → At-Risk)
    CatEngine->>CatEngine: Add "At-Risk" category, keep existing categories
    CatEngine->>CatEngine: Resolve new set: [Individual, VIP, At-Risk]

    CatEngine-)PolicyEngine: CategoryAssigned (At-Risk added)
    PolicyEngine->>PolicyEngine: Evaluate policies for [Individual, VIP, At-Risk]
    PolicyEngine-->>CampaignSvc: Actions: retention offer (30% discount), priority support flag, manager callback

    CampaignSvc->>CampaignSvc: Create retention action
    CampaignSvc->>ChannelSvc: Dispatch retention offer (customer's preferred channel)
    ChannelSvc->>ChannelSvc: Check channel preference + frequency cap
    ChannelSvc-->>CampaignSvc: Dispatched via WhatsApp (customer preference)
```

## Synchronous API Calls (Cross-Service)

```mermaid
sequenceDiagram
    participant Agent as Agent Portal
    participant CustSvc as Customer Management
    participant CatEngine as Category Engine
    participant PolicyEngine as Policy Engine
    participant CampaignSvc as Campaign Service
    participant BSS as BSS Integration

    Note over Agent: Agent opens customer 360-view
    Agent->>CustSvc: GET /customers/{id}/360
    CustSvc->>CatEngine: GET /categories/resolve/{customerId}
    CatEngine-->>CustSvc: [Individual, VIP, Active, Digital]
    CustSvc->>BSS: GET /billing/summary/{customerId}
    BSS-->>CustSvc: Current plan, ARPU, balance
    CustSvc-->>Agent: Complete 360-view

    Note over Agent: Agent wants to see applicable offers
    Agent->>PolicyEngine: POST /evaluate (customerId, context: "agent_interaction")
    PolicyEngine->>CatEngine: GET /categories/resolve/{customerId}
    CatEngine-->>PolicyEngine: [Individual, VIP, Active, Digital]
    PolicyEngine-->>Agent: Applicable offers [upgrade_5G, loyalty_bonus, referral_reward]

    Note over Agent: Agent applies offer
    Agent->>CampaignSvc: POST /offers/{offerId}/apply (customerId)
    CampaignSvc->>BSS: Apply discount/plan change
    BSS-->>CampaignSvc: Confirmed
    CampaignSvc-->>Agent: Offer applied successfully
```

## External System Integration

```mermaid
graph TB
    subgraph Platform["Telecom CRM Platform"]
        INTEGRATION[BSS/OSS Integration Service]
        ORDER[Order Management]
        CHANNEL[Channel Orchestration]
        ANALYTICS[Analytics Service]
        NOTIF[Notification Service]
    end

    subgraph External["External Systems"]
        BSS[BSS — Billing<br/>Rating, invoicing, payment, balance]
        OSS[OSS — Network<br/>Provisioning, activation, portability]
        PRODUCT[Product Catalog<br/>Plans, add-ons, devices, bundles]
        PAY_GW[Payment Gateway<br/>Card, bank transfer, digital wallet]
        SMS_GW[SMS Gateway<br/>Bulk + transactional SMS]
        EMAIL_GW[Email Service<br/>Transactional + marketing email]
        PUSH[Push Notification<br/>FCM/APNs]
        WHATSAPP[WhatsApp Business API<br/>Conversational + broadcast]
        REGULATORY[Regulatory Authority<br/>ANATEL — compliance, portability DB]
        ML_PLATFORM[Data Analytics / ML<br/>Churn models, segmentation, scoring]
    end

    INTEGRATION -->|REST/SOAP + Circuit Breaker<br/>Billing queries, charges, credits| BSS
    INTEGRATION -->|REST/SOAP + Circuit Breaker<br/>SIM activation, number port, provisioning| OSS
    INTEGRATION -->|REST API<br/>Plan catalog sync, eligibility check| PRODUCT
    INTEGRATION -->|REST + PCI-DSS<br/>Payment processing, refunds| PAY_GW

    CHANNEL -->|SMPP/REST API<br/>Campaign + transactional SMS| SMS_GW
    CHANNEL -->|SMTP/REST API<br/>Email delivery| EMAIL_GW
    CHANNEL -->|REST API<br/>Push notifications| PUSH
    CHANNEL -->|WhatsApp Cloud API<br/>Rich messaging| WHATSAPP

    ANALYTICS -->|REST + mTLS<br/>Regulatory filings, portability reports| REGULATORY
    ANALYTICS -->|Kafka + REST<br/>Events out, scores in| ML_PLATFORM
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Pub/Sub (Kafka)** | Inter-service event notifications | Loose coupling, independent scaling, event replay for analytics |
| **Synchronous gRPC** | Category resolution + policy evaluation in real-time flows | Low-latency, type-safe, required during customer interactions |
| **API Gateway** | All client-to-service calls | Centralized auth, rate limiting, channel/partner routing |
| **Circuit Breaker** | BSS/OSS and external system calls | Telecom backend systems have variable SLAs; graceful degradation |
| **Saga Pattern** | Order processing (customer creation → provisioning → billing activation) | Distributed transaction consistency with compensation |
| **CQRS** | Analytics/reporting service | Separate read models for dashboards, campaign metrics, regulatory |
| **Retry + Dead Letter Queue** | Failed event processing, BSS timeouts | Guaranteed delivery with error isolation |
| **Outbox Pattern** | Customer lifecycle events | Exactly-once event publishing with transactional guarantees |
| **Anti-Corruption Layer** | BSS/OSS integration | Isolate legacy SOAP/proprietary protocols from modern domain model |
| **Event Sourcing** | Customer lifecycle + category transitions | Complete audit trail, point-in-time reconstruction, temporal queries |
| **Webhook** | Partner order notifications, payment confirmations | Real-time partner integration without polling |
| **Batch + Streaming** | Analytics scoring (daily batch) + real-time triggers | Balance between ML model freshness and operational cost |

## Data Flow Metrics

| Stage | Expected Throughput | Target Latency |
|-------|-------------------|----------------|
| Customer 360-view assembly | 50,000 requests/hour | < 500ms |
| Category resolution (single customer) | 100,000 resolutions/hour | < 100ms |
| Policy evaluation (single customer) | 100,000 evaluations/hour | < 200ms |
| Campaign action dispatch | 500,000 messages/hour (peak campaign) | < 5 seconds |
| Customer creation (onboarding) | 10,000 activations/hour (peak promo) | < 3 seconds |
| BSS billing query | 200,000 queries/hour | < 1 second |
| Order provisioning (SIM activation) | 5,000 activations/hour | < 30 seconds |
| Churn score batch update | 10M customers/nightly batch | < 4 hours |
| Real-time event processing | 1M events/hour | < 1 second (end-to-end) |
| Regulatory report generation | Daily/monthly/quarterly | < 1 hour |
