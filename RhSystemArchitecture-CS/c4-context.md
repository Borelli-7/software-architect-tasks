# C4 System Context Diagram — Bank HRMS

## Description
Shows the HRMS system in context with its external actors and neighboring systems.

```mermaid
C4Context
    title System Context Diagram — Bank HRMS

    Person(hrManager, "HR Manager", "Manages HR operations, payroll, career planning")
    Person(hrAdmin, "HR Administrator", "Handles employee data entry and updates")
    Person(supervisor, "Supervisor/Manager", "Conducts evaluations, approves leave")
    Person(employee, "Bank Employee", "Views payslips, submits leave, self-service")

    System(hrms, "HRMS", "Human Resources Management System — manages employee lifecycle, evaluations, training, career planning, and payroll for 16,000+ employees across 2,400 branches")

    System_Ext(ad, "Active Directory", "Corporate identity and authentication provider")
    System_Ext(banking_core, "Core Banking System", "Employee banking accounts for salary disbursement")
    System_Ext(tax_authority, "Tax Authority", "Regulatory tax reporting and withholding")
    System_Ext(training_providers, "External Training Providers", "Third-party training course catalogs and enrollment")
    System_Ext(email, "Email System", "Notifications, payslips, evaluation reminders")

    Rel(hrManager, hrms, "Manages payroll, career plans, training programs")
    Rel(hrAdmin, hrms, "Creates/updates employee records")
    Rel(supervisor, hrms, "Conducts evaluations, approves requests")
    Rel(employee, hrms, "Self-service: leave, payslips, objectives")

    Rel(hrms, ad, "Authenticates users, syncs org structure")
    Rel(hrms, banking_core, "Sends salary payment instructions")
    Rel(hrms, tax_authority, "Submits tax declarations")
    Rel(hrms, training_providers, "Fetches course catalogs, enrolls employees")
    Rel(hrms, email, "Sends notifications and documents")
```

## Key Observations

- **4 actor types** with different access levels and needs
- **5 external system integrations** requiring secure APIs
- The HRMS is the central system connecting people management to financial and regulatory systems
- All communication crosses the bank's security boundary, requiring encrypted channels
