# Security Architecture Overlay — Bank HRMS

## Description
Cross-cutting security architecture showing network zones, authentication flows, encryption boundaries, and access control enforcement points.

## Network Security Zones

```mermaid
graph TB
    subgraph Internet["Internet Zone"]
        browser[Browser/PWA]
    end

    subgraph DMZ["DMZ — Perimeter Zone"]
        waf[Web Application Firewall]
        cdn[CDN / Static Assets]
        lb[Load Balancer]
    end

    subgraph AppZone["Application Zone"]
        gw[API Gateway<br/>Token validation, rate limiting]
        idp[Identity Provider<br/>Keycloak — OAuth2/OIDC]
        
        subgraph GeneralServices["General Services Subnet"]
            emp_svc[Employee Service]
            eval_svc[Evaluation Service]
            career_svc[Career Service]
            training_svc[Training Service]
            notif_svc[Notification Service]
            report_svc[Reporting Service]
        end
        
        subgraph RestrictedServices["Restricted Services Subnet"]
            payroll_svc[Payroll Service<br/>HR-Payroll role only]
        end
    end

    subgraph DataZone["Data Zone — No Direct External Access"]
        subgraph GeneralData["General Data"]
            emp_db[(Employee DB)]
            eval_db[(Evaluation DB)]
            training_db[(Training DB)]
            cache[(Redis Cache)]
            broker[Kafka Broker]
        end
        
        subgraph SensitiveData["Sensitive Data — HSM Encrypted"]
            payroll_db[(Payroll DB<br/>Field-level encryption)]
            audit_db[(Audit Log DB<br/>Append-only)]
        end
    end

    subgraph External["External Systems"]
        ad[Active Directory]
        banking[Core Banking]
        tax[Tax Authority]
    end

    browser --> waf
    waf --> cdn
    waf --> lb
    lb --> gw
    gw --> idp
    gw --> GeneralServices
    gw --> RestrictedServices
    
    GeneralServices --> GeneralData
    RestrictedServices --> SensitiveData
    RestrictedServices --> broker
    
    idp --> ad
    payroll_svc --> banking
    payroll_svc --> tax
```

## Authentication & Authorization Flow

```mermaid
sequenceDiagram
    participant User
    participant SPA as Web App (SPA)
    participant GW as API Gateway
    participant IDP as Identity Provider
    participant AD as Active Directory
    participant Svc as Backend Service

    User->>SPA: Access HRMS
    SPA->>IDP: Redirect to login (OIDC)
    IDP->>AD: Validate credentials (LDAP)
    AD-->>IDP: User authenticated + groups
    IDP->>IDP: Issue JWT (roles, permissions, branch)
    IDP-->>SPA: Access token + Refresh token
    
    SPA->>GW: API request + Bearer token
    GW->>GW: Validate JWT signature & expiry
    GW->>GW: Check rate limits
    GW->>GW: Extract roles, check route-level ACL
    
    alt Authorized
        GW->>Svc: Forward request + user context
        Svc->>Svc: Fine-grained ABAC check
        Svc-->>GW: Response
        GW-->>SPA: Response
    else Unauthorized
        GW-->>SPA: 403 Forbidden
    end
```

## RBAC Model

```mermaid
graph LR
    subgraph Roles
        R1[EMPLOYEE]
        R2[SUPERVISOR]
        R3[HR_ADMIN]
        R4[HR_MANAGER]
        R5[HR_PAYROLL]
        R6[SYSTEM_ADMIN]
    end

    subgraph Modules
        M1[Employee Self-Service]
        M2[Employee Admin]
        M3[Performance Evaluation]
        M4[Career Planning]
        M5[Training Program]
        M6[Payroll]
        M7[Reporting]
        M8[System Config]
    end

    R1 -->|read own| M1
    R1 -->|participate| M3
    R1 -->|view own| M5
    
    R2 -->|read team| M2
    R2 -->|evaluate team| M3
    R2 -->|approve| M5
    
    R3 -->|full CRUD| M2
    R3 -->|manage| M3
    R3 -->|manage| M5
    
    R4 -->|full access| M2
    R4 -->|full access| M3
    R4 -->|full access| M4
    R4 -->|full access| M5
    R4 -->|view reports| M7
    
    R5 -->|full access| M6
    R5 -->|view| M2
    
    R6 -->|full access| M8
    R6 -->|audit| M7
```

## Encryption Strategy

| Layer | Method | Key Management |
|-------|--------|----------------|
| In transit (external) | TLS 1.3 | PKI certificates, auto-rotation |
| In transit (internal) | mTLS | Service mesh certificates (Istio/Linkerd) |
| At rest (general) | AES-256 (database-level) | Key vault managed |
| At rest (payroll) | AES-256 (field-level) | HSM-backed keys |
| Documents | AES-256 (object-level) | Per-document keys wrapped by master key |
| Backups | AES-256 | Offline master key + split knowledge |

## Compliance Controls

| Control | Implementation |
|---------|----------------|
| **Data residency** | All data stored in-country; no cross-border replication |
| **Right to erasure** | Soft-delete with retention policy; hard-delete after legal hold |
| **Access logging** | All API calls logged with user, action, resource, timestamp |
| **Penetration testing** | Quarterly external pen tests; continuous internal scanning |
| **Incident response** | Automated alerts on anomalous access patterns |
| **Key rotation** | Encryption keys rotated every 90 days |
