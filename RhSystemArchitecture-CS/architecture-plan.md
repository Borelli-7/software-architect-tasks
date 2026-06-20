# HRMS Architecture Plan — National Bank

## Executive Summary

Enterprise Human Resources Management System (HRMS) for a national bank with 2,400 branches and 16,000+ employees. The architecture follows a modular, API-first approach using microservices deployed on a cloud-native platform, accessible via web browsers across heterogeneous environments.

---

## 1. Architectural Decisions

### 1.1 Cross-Platform Strategy
| Decision | Rationale |
|----------|-----------|
| **Web-based SPA frontend** | Browser-agnostic; works on Windows, Linux, macOS, thin clients |
| **API-first backend (REST/gRPC)** | Decouples frontend from business logic; enables mobile access |
| **Containerized microservices** | Consistent deployment across environments; horizontal scaling |
| **Progressive Web App (PWA)** | Offline capability for branches with unreliable connectivity |

### 1.2 Security Strategy
| Decision | Rationale |
|----------|-----------|
| **Zero-trust network model** | No implicit trust; verify every request regardless of origin |
| **OAuth 2.0 + OpenID Connect** | Industry-standard identity federation for distributed users |
| **mTLS for service-to-service** | Encrypted internal communication |
| **AES-256 encryption at rest** | Banking-grade data protection |
| **TLS 1.3 in transit** | All network traffic encrypted |
| **Data residency in-country** | Compliance with national banking regulations |

### 1.3 Access Control Strategy
| Decision | Rationale |
|----------|-----------|
| **Role-Based Access Control (RBAC)** | Map organizational roles to system permissions |
| **Attribute-Based Access Control (ABAC)** | Fine-grained control based on department, branch, seniority |
| **Module-level authorization** | Payroll/Career Planning restricted to authorized HR staff |
| **Audit logging** | All access attempts logged for compliance |

---

## 2. Technology Stack

| Layer | Technology | Justification |
|-------|-----------|---------------|
| Frontend | Angular / React | Enterprise SPA, cross-platform via browser |
| API Gateway | Kong / AWS API Gateway | Rate limiting, auth, routing |
| Backend Services | Java Spring Boot / .NET 8 | Enterprise-grade, strong typing, ecosystem |
| Message Broker | Apache Kafka | Event-driven async communication between modules |
| Database | PostgreSQL (primary) | ACID compliance, JSON support, scalability |
| Cache | Redis | Session management, performance optimization |
| Identity Provider | Keycloak / Azure AD | OIDC/SAML federation, MFA support |
| Container Orchestration | Kubernetes | Auto-scaling, self-healing, multi-region |
| Monitoring | Prometheus + Grafana | Observability, alerting |
| CI/CD | GitLab CI / GitHub Actions | Automated testing and deployment |

---

## 3. Module Breakdown

### 3.1 Employee Administration
- Employee CRUD lifecycle (hire, update, terminate)
- Document management (contracts, ID, certifications)
- Organizational structure (branches, departments, reporting lines)
- Integration with Active Directory for identity provisioning

### 3.2 Performance Evaluation
- Semi-annual evaluation campaign management
- 360° feedback collection
- Objective setting and tracking (quarterly)
- Scoring and analytics dashboard
- Notification engine for evaluation deadlines

### 3.3 Career Path Planning
- Career trajectory modeling
- Competency matrix management
- Succession planning
- Post-evaluation career discussion records
- Gap analysis reports

### 3.4 Training Program
- Training needs assessment per department
- Course catalog management
- External provider integration (API/manual)
- Enrollment and attendance tracking
- Budget allocation and tracking
- Certification management

### 3.5 Payroll
- Salary computation engine (base + bonuses + commissions)
- Leave/vacation management and accrual
- Tax calculation and withholding
- Payment file generation (bank transfers)
- Payslip generation and distribution
- Regulatory compliance reporting

---

## 4. Diagram Inventory

| # | Diagram | Format | File |
|---|---------|--------|------|
| 1 | C4 System Context | Mermaid | `c4-context.md` |
| 2 | C4 Container | Mermaid | `c4-container.md` |
| 3 | C4 Component — Employee Admin | Mermaid | `c4-component-employee-admin.md` |
| 4 | C4 Component — Performance Eval | Mermaid | `c4-component-performance-eval.md` |
| 5 | C4 Component — Payroll | Mermaid | `c4-component-payroll.md` |
| 6 | Security Architecture Overlay | Mermaid | `security-architecture.md` |
| 7 | Integration Patterns | Mermaid | `integration-patterns.md` |

---

## 5. Execution Order

1. **Phase 1 — Context** → `c4-context.md` (system boundaries, external actors)
2. **Phase 2 — Containers** → `c4-container.md` (runtime units, databases, message brokers)
3. **Phase 3 — Components** → Module-level component diagrams
4. **Phase 4 — Cross-cutting** → Security overlay + integration patterns
5. **Phase 5 — Verification** → Review against constraints (cross-platform, security, RBAC)
