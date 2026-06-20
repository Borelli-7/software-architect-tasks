# Integration Patterns — Bank HRMS

## Description
Defines how the 5 HRMS modules communicate with each other and with external systems, using event-driven and synchronous patterns.

## Event-Driven Architecture (Inter-Module)

```mermaid
graph LR
    subgraph Producers
        EMP[Employee Service]
        EVAL[Evaluation Service]
        LEAVE[Leave Management]
    end

    subgraph Kafka["Apache Kafka — Event Bus"]
        T1[employee.lifecycle]
        T2[evaluation.completed]
        T3[leave.approved]
        T4[org.structure.changed]
    end

    subgraph Consumers
        PAY[Payroll Service]
        CAREER[Career Service]
        TRAIN[Training Service]
        NOTIF[Notification Service]
        REPORT[Reporting Service]
    end

    EMP -->|EmployeeCreated<br/>EmployeeUpdated<br/>EmployeeTerminated| T1
    EMP -->|DepartmentChanged<br/>BranchReorganized| T4
    EVAL -->|EvaluationCompleted<br/>ObjectivesSet| T2
    LEAVE -->|LeaveApproved<br/>LeaveRejected| T3

    T1 --> PAY
    T1 --> CAREER
    T1 --> TRAIN
    T1 --> NOTIF
    
    T2 --> CAREER
    T2 --> TRAIN
    T2 --> NOTIF
    T2 --> REPORT
    
    T3 --> PAY
    T3 --> NOTIF
    
    T4 --> EVAL
    T4 --> REPORT
```

## Synchronous API Calls (Cross-Module Queries)

```mermaid
sequenceDiagram
    participant EVAL as Evaluation Service
    participant EMP as Employee Service
    participant CAREER as Career Service
    participant TRAIN as Training Service

    Note over EVAL: Evaluation campaign starts
    EVAL->>EMP: GET /employees?department={id}&active=true
    EMP-->>EVAL: List of active employees in department
    
    Note over EVAL: Evaluation completed
    EVAL->>CAREER: POST /career-discussions (evaluation results)
    CAREER-->>EVAL: Career plan updated
    
    CAREER->>TRAIN: POST /training-needs (skill gaps identified)
    TRAIN-->>CAREER: Training recommendations
```

## External System Integration

```mermaid
graph TB
    subgraph HRMS["HRMS Boundary"]
        EMP[Employee Service]
        PAY[Payroll Service]
        TRAIN[Training Service]
        NOTIF[Notification Service]
        IDP[Identity Provider]
    end

    subgraph External["External Systems"]
        AD[Active Directory]
        BANK[Core Banking System]
        TAX[Tax Authority]
        PROVIDERS[Training Providers]
        EMAIL[Email/SMS Gateway]
    end

    IDP -->|LDAP/SAML<br/>User auth, group sync| AD
    EMP -->|Provisioning<br/>Create/disable accounts| AD
    
    PAY -->|SFTP + PGP<br/>Payment batch files| BANK
    PAY -->|REST API + mTLS<br/>Tax declarations| TAX
    
    TRAIN -->|REST API<br/>Course catalog sync| PROVIDERS
    TRAIN -->|Webhook<br/>Enrollment confirmations| PROVIDERS
    
    NOTIF -->|SMTP/TLS<br/>Emails| EMAIL
    NOTIF -->|SMS API<br/>Urgent notifications| EMAIL
```

## Integration Patterns Summary

| Pattern | Usage | Rationale |
|---------|-------|-----------|
| **Event Sourcing** | Employee lifecycle events | Complete audit trail, temporal queries |
| **Pub/Sub (Kafka)** | Inter-module notifications | Loose coupling, independent scaling |
| **API Gateway** | All client-to-service calls | Centralized auth, rate limiting, routing |
| **Circuit Breaker** | External system calls | Fault tolerance for banking/tax APIs |
| **Saga Pattern** | Payment processing | Distributed transaction across payroll + banking |
| **CQRS** | Reporting service | Separate read models optimized for analytics |
| **Retry + Dead Letter** | Failed event processing | Guaranteed delivery with error isolation |
| **Batch Integration** | Payroll to Core Banking | Nightly payment file generation via SFTP |
| **Webhook** | Training providers | Real-time enrollment status updates |

## Data Flow — Monthly Payroll Run

```mermaid
sequenceDiagram
    participant HR as HR Manager
    participant PAY as Payroll Service
    participant EMP as Employee Service
    participant LEAVE as Leave Data
    participant TAX as Tax Engine
    participant BANK as Core Banking
    participant NOTIF as Notification Service

    HR->>PAY: Trigger monthly payroll run
    PAY->>EMP: Fetch active employees + salary structures
    EMP-->>PAY: Employee list with compensation data
    PAY->>LEAVE: Fetch approved leave for period
    LEAVE-->>PAY: Leave records (unpaid leave, overtime)
    
    loop For each employee
        PAY->>PAY: Calculate gross (base + bonus + commission)
        PAY->>TAX: Apply tax rules and deductions
        TAX-->>PAY: Net salary + tax withholding
    end
    
    PAY->>PAY: Generate payment batch file
    PAY->>HR: Request dual authorization
    HR->>PAY: Second manager approves
    PAY->>BANK: Submit payment file (SFTP/PGP encrypted)
    BANK-->>PAY: Acknowledgment + reference numbers
    
    PAY->>PAY: Generate payslips (PDF, digitally signed)
    PAY->>NOTIF: Distribute payslips to employees
    NOTIF-->>PAY: Delivery confirmation
```
