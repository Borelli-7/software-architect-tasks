# C4 System Context Diagram — Insurance Policy Management Platform

## Description
Shows the Insurance Policy Management Platform in context with its external actors (users) and neighboring systems it integrates with.

```mermaid
C4Context
    title System Context Diagram — Insurance Policy Management Platform

    Person(policyAdmin, "Policy Administrator", "Creates and manages insurance policies, processes endorsements and renewals")
    Person(bizAnalyst, "Industry Business Analyst", "Defines and maintains industry-specific business rules and coverage catalogs")
    Person(clientHR, "Client HR Manager", "Enrolls employees, manages group policies, submits claims on behalf of employees")
    Person(insuredEmployee, "Insured Employee", "Views coverage, submits claims, updates beneficiaries")
    Person(claimsAdjuster, "Claims Adjuster", "Reviews and adjudicates insurance claims")
    Person(complianceOfficer, "Compliance Officer", "Audits operations, generates regulatory reports")

    System(ipm, "Insurance Policy Management Platform", "Manages insurance policies for client companies' employees across multiple industries — finance, manufacturing, technology, engineering, healthcare")

    System_Ext(clientHRSystems, "Client HR Systems", "Employee census data, organizational structures per client company")
    System_Ext(paymentGateway, "Payment Gateway", "Processes premium payments and claim disbursements")
    System_Ext(reinsurance, "Reinsurance Partners", "Ceded risk, treaty management, facultative placements")
    System_Ext(regulatoryAPI, "Regulatory Authority", "Insurance regulatory filings, compliance reporting")
    System_Ext(govBenefits, "Government Benefits Portal", "Tax/benefits reporting, social security integration")
    System_Ext(documentMgmt, "Document Management System", "Long-term document archival and retrieval")
    System_Ext(emailSMS, "Email/SMS Gateway", "Multi-channel notification delivery")

    Rel(policyAdmin, ipm, "Creates policies, processes endorsements, manages renewals")
    Rel(bizAnalyst, ipm, "Configures industry rules, coverage catalogs, eligibility criteria")
    Rel(clientHR, ipm, "Enrolls employees, views group policies, submits bulk claims")
    Rel(insuredEmployee, ipm, "Views coverage, submits claims, manages beneficiaries")
    Rel(claimsAdjuster, ipm, "Reviews claims, approves/denies, flags fraud")
    Rel(complianceOfficer, ipm, "Audits transactions, generates regulatory reports")

    Rel(ipm, clientHRSystems, "Syncs employee census data (batch + real-time)")
    Rel(ipm, paymentGateway, "Collects premiums, disburses claim payments")
    Rel(ipm, reinsurance, "Reports ceded risk, receives treaty terms")
    Rel(ipm, regulatoryAPI, "Submits filings, receives compliance updates")
    Rel(ipm, govBenefits, "Reports benefits, validates social security")
    Rel(ipm, documentMgmt, "Archives policies, certificates, claim documents")
    Rel(ipm, emailSMS, "Sends notifications, reminders, claim status updates")
```

## Key Observations

- **6 actor types** representing internal insurance staff, client-side users, and insured individuals
- **7 external system integrations** spanning client data sync, payments, regulatory, and document management
- The platform is the central hub connecting policy administration to client companies across multiple industries
- Industry Business Analyst is a key differentiator — they configure rules without developer involvement
- Client HR Managers interact on behalf of their employees, creating a B2B2C model
- Regulatory compliance is a first-class concern with direct integration to government authorities
