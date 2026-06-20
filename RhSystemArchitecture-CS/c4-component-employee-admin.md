# C4 Component Diagram — Employee Administration Service

## Description
Internal components of the Employee Administration microservice, showing controllers, domain services, and repository layers.

```mermaid
C4Component
    title Component Diagram — Employee Administration Service

    Container_Boundary(employee_svc, "Employee Service") {
        Component(emp_controller, "Employee Controller", "REST Controller", "Handles HTTP requests for employee CRUD operations")
        Component(org_controller, "Organization Controller", "REST Controller", "Manages branches, departments, reporting hierarchy")
        Component(doc_controller, "Document Controller", "REST Controller", "Upload/download employee documents")
        
        Component(emp_service, "Employee Domain Service", "Service Layer", "Business logic: hiring, updates, termination workflows")
        Component(org_service, "Organization Service", "Service Layer", "Org structure validation, branch management")
        Component(doc_service, "Document Service", "Service Layer", "Document storage, versioning, access control")
        Component(event_publisher, "Event Publisher", "Kafka Producer", "Publishes EmployeeCreated, EmployeeUpdated, EmployeeTerminated events")
        
        Component(emp_repo, "Employee Repository", "JPA/EF Repository", "CRUD operations on employee table")
        Component(org_repo, "Organization Repository", "JPA/EF Repository", "Branch and department data access")
        Component(doc_store, "Document Store", "Object Storage Client", "Stores documents in encrypted blob storage")
    }

    ContainerDb(employee_db, "Employee DB", "PostgreSQL", "Employee records")
    Container(blob, "Document Storage", "S3/MinIO", "Encrypted document blobs")
    Container(broker, "Message Broker", "Kafka", "Event bus")
    Container(idp, "Identity Provider", "Keycloak", "Token validation")

    Rel(emp_controller, emp_service, "Delegates to")
    Rel(org_controller, org_service, "Delegates to")
    Rel(doc_controller, doc_service, "Delegates to")
    
    Rel(emp_service, emp_repo, "Uses")
    Rel(emp_service, event_publisher, "Publishes lifecycle events")
    Rel(org_service, org_repo, "Uses")
    Rel(doc_service, doc_store, "Stores/retrieves")
    
    Rel(emp_repo, employee_db, "SQL queries", "TLS")
    Rel(org_repo, employee_db, "SQL queries", "TLS")
    Rel(doc_store, blob, "PUT/GET objects", "HTTPS")
    Rel(event_publisher, broker, "Produces messages", "Kafka/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `EmployeeCreated` | New hire completed | Payroll, Training, Career |
| `EmployeeUpdated` | Profile/position change | Payroll, Evaluation |
| `EmployeeTerminated` | Offboarding completed | Payroll, All modules |
| `OrgStructureChanged` | Branch/dept reorganization | Evaluation, Reporting |
