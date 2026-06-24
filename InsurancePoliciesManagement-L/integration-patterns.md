# Integration Patterns — Insurance Policy Management Platform

## Description
Defines how the platform's modules communicate with each other and with external systems, using event-driven and synchronous patterns to support multi-industry policy management.

## Event-Driven Architecture (Inter-Module)

```mermaid
graph LR
    subgraph Producers
        POLICY[Policy Lifecycle Service]
        CLAIMS[Claims Processing Service]
        BILLING[Billing & Payments Service]
        RULES[Industry Rules Engine]
        CLIENT[Client Management Service]
    end

    subgraph Kafka["Apache Kafka — Event Bus"]
        T1[policy.lifecycle]
        T2[claims.processing]
        T3[billing.payments]
        T4[rules.management]
        T5[client.census]
    end

    subgraph Consumers
        NOTIF[Notification Service]
        DOC[Document Generation]
        REPORT[Reporting Service]
        BILLING_C[Billing Service]
        CLAIMS_C[Claims Service]
    end

    POLICY -->|PolicyCreated<br/>PolicyActivated<br/>PolicyEndorsed<br/>PolicyRenewed<br/>PolicyCancelled| T1
    CLAIMS -->|ClaimSubmitted<br/>ClaimApproved<br/>ClaimDenied<br/>FraudFlagged| T2
    BILLING -->|PremiumCollected<br/>PaymentFailed<br/>ClaimDisbursed| T3
    RULES -->|RuleActivated<br/>RuleDeprecated<br/>AdapterDeployed| T4
    CLIENT -->|EmployeeEnrolled<br/>EmployeeTerminated<br/>CensusUpdated| T5

    T1 --> NOTIF
    T1 --> DOC
    T1 --> REPORT
    T1 --> BILLING_C

    T2 --> NOTIF
    T2 --> DOC
    T2 --> REPORT
    T2 --> BILLING_C

    T3 --> NOTIF
    T3 --> REPORT

    T4 --> REPORT

    T5 --> CLAIMS_C
    T5 --> BILLING_C
    T5 --> REPORT
```

## End-to-End Policy Creation Flow (Sequence)

```mermaid
sequenceDiagram
    participant ClientHR as Client HR Manager
    participant Portal as Client Portal
    participant GW as API Gateway
    participant Client as Client Service
    participant Policy as Policy Lifecycle Service
    participant Rules as Industry Rules Engine
    participant Billing as Billing Service
    participant DocGen as Document Generation
    participant Notif as Notification Service

    ClientHR->>Portal: Request new group policy
    Portal->>GW: POST /policies (industryId, employees, coverage)
    GW->>GW: Validate token, resolve tenant
    GW->>Policy: Create policy request

    Policy->>Client: GET /clients/{id}/employees (verify census)
    Client-->>Policy: Employee list confirmed

    Policy->>Rules: EvaluateRules(industryId, "policy.creation", context)
    Rules-->>Policy: Eligible coverages, required fields, constraints

    Policy->>Policy: Create policy (Draft state)
    Policy->>Billing: Calculate premium (coverage, employees, industry rules)
    Billing->>Rules: EvaluateRules(industryId, "premium.calculation", context)
    Rules-->>Billing: Premium factors, discounts, surcharges
    Billing-->>Policy: Premium quote

    Policy-->>Portal: Policy draft + premium quote
    ClientHR->>Portal: Accept and pay
    Portal->>GW: POST /policies/{id}/activate
    GW->>Policy: Activate policy

    Policy->>Policy: Transition Draft → Active
    Policy->>Billing: Generate invoice
    Policy-->>Portal: Policy activated

    Note over Policy: Publishes PolicyActivated event
    Policy-)DocGen: PolicyActivated event
    DocGen->>DocGen: Generate policy certificate
    Policy-)Notif: PolicyActivated event
    Notif->>ClientHR: Email confirmation + certificate
```

## Industry Onboarding Flow

```mermaid
sequenceDiagram
    participant BA as Industry Business Analyst
    participant Builder as Rule Definition Interface
    participant Rules as Rules Engine
    participant Sandbox as Testing Sandbox
    participant Senior as Senior Business Analyst
    participant Platform as Platform

    BA->>Builder: Define industry coverage types
    BA->>Builder: Define eligibility rules
    BA->>Builder: Define benefit calculation rules
    BA->>Builder: Define premium factors
    Builder->>Rules: Save rule set (Draft)

    BA->>Sandbox: Run test scenarios
    Sandbox->>Rules: Execute rules against synthetic data
    Rules-->>Sandbox: Results
    Sandbox-->>BA: Test report (pass/fail)

    alt Tests pass
        BA->>Senior: Request approval
        Senior->>Senior: Cross-industry consistency review
        Senior->>Rules: Approve → Activate rule set
        Rules->>Platform: Publish RuleActivated event
        Note over Platform: New industry now available for policy creation
    else Tests fail
        Sandbox-->>BA: Failure details
        BA->>Builder: Revise rules
    end
```

## Synchronous API Calls (Cross-Service)

```mermaid
sequenceDiagram
    participant Policy as Policy Lifecycle Service
    participant Rules as Industry Rules Engine
    participant Client as Client Management Service
    participant Billing as Billing & Payments Service

    Note over Policy: Policy creation requires sync validation
    Policy->>Rules: POST /evaluate (industryId, ruleType, context)
    Rules-->>Policy: EvaluationResult (eligible, constraints)

    Note over Policy: Census verification
    Policy->>Client: GET /clients/{clientId}/employees?active=true
    Client-->>Policy: Employee list with coverage status

    Note over Billing: Premium calculation requires rules
    Billing->>Rules: POST /evaluate (industryId, "premium", factors)
    Rules-->>Billing: Premium components (base, riders, discounts)

    Note over Billing: Payment processing
    Billing->>Billing: Generate invoice
```

## External System Integration

```mermaid
graph TB
    subgraph Platform["Insurance Policy Management Platform"]
        CLIENT[Client Management Service]
        BILLING[Billing & Payments Service]
        REPORT[Reporting Service]
        NOTIF[Notification Service]
        DOC[Document Generation]
    end

    subgraph External["External Systems"]
        HR_SYS[Client HR Systems<br/>Per-industry varied formats]
        PAY_GW[Payment Gateway<br/>Premium + Claims disbursement]
        REINSURE[Reinsurance Partners<br/>Treaty + Facultative]
        REG[Regulatory Authority<br/>Compliance filings]
        GOV[Government Portal<br/>Tax/Benefits reporting]
        EMAIL[Email/SMS Gateway]
        DOC_ARCH[Document Archive<br/>Long-term storage]
    end

    CLIENT -->|REST API + SFTP<br/>Census sync (batch nightly + webhook real-time)| HR_SYS
    BILLING -->|REST API + PCI-DSS<br/>Payment processing| PAY_GW
    BILLING -->|REST API + mTLS<br/>Ceded risk reporting| REINSURE
    REPORT -->|REST API + mTLS<br/>Quarterly filings| REG
    REPORT -->|SFTP + PGP<br/>Annual benefits reporting| GOV
    NOTIF -->|SMTP/TLS + SMS API<br/>Multi-channel delivery| EMAIL
    DOC -->|S3 API<br/>Document archival| DOC_ARCH
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Pub/Sub (Kafka)** | Inter-service event notifications | Loose coupling, independent scaling, event replay |
| **Synchronous gRPC** | Rule evaluation during policy/claim creation | Low-latency, type-safe, required for transaction integrity |
| **API Gateway** | All client-to-service calls | Centralized auth, rate limiting, tenant resolution |
| **Circuit Breaker** | External system calls (HR, Payment, Regulatory) | Fault tolerance, graceful degradation |
| **Saga Pattern** | Claims processing (validate → adjudicate → pay) | Distributed transaction consistency |
| **CQRS** | Reporting service | Separate read models for analytics and compliance |
| **Retry + Dead Letter Queue** | Failed event processing | Guaranteed delivery with error isolation |
| **Batch Integration** | Census sync (nightly), regulatory reports (quarterly) | Bulk data transfer for non-real-time scenarios |
| **Webhook** | Real-time census updates from client HR | Immediate enrollment/termination propagation |
| **Event Sourcing** | Policy lifecycle | Complete audit trail, point-in-time reconstruction |

## Data Flow Metrics

| Stage | Expected Throughput | Target Latency |
|-------|-------------------|----------------|
| Policy creation (incl. rule evaluation) | 500 policies/hour peak | < 3 seconds |
| Rule evaluation (single call) | 10,000 evaluations/hour | < 200ms |
| Claim submission to acknowledgment | 200 claims/hour | < 1 second |
| Claim auto-adjudication | 70% of eligible claims | < 5 seconds |
| Premium calculation | 1,000 quotes/hour | < 500ms |
| Census sync (batch) | 50,000 records/nightly batch | < 2 hours |
| Notification delivery | 5,000 notifications/hour | < 30 seconds |
