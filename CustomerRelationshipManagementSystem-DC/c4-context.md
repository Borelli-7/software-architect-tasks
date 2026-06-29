# C4 System Context Diagram — Telecom CRM Platform

## Description
Shows the CRM platform in context with its external actors (users) and neighboring systems it integrates with.

```mermaid
C4Context
    title System Context Diagram — Telecom CRM Platform

    Person(salesRep, "Sales Representative", "Sells plans, manages customer accounts, processes upgrades across retail stores and call centers")
    Person(marketingAnalyst, "Marketing Business Analyst", "Defines customer categories, configures marketing policies, designs campaigns and offers")
    Person(supportAgent, "Support Agent", "Handles customer inquiries, complaints, retention actions via call center and chat")
    Person(channelManager, "Channel Manager", "Manages dealer/partner relationships, configures partner-specific policies")
    Person(customer, "Customer", "Individual, business, or government customer — manages account, views offers, interacts via app/web/store")
    Person(complianceOfficer, "Compliance Officer", "Audits data access, manages consent policies, generates regulatory reports")

    System(crm, "Telecom CRM Platform", "Manages customer relationships across multiple overlapping categories — resolves marketing policies and actions dynamically based on category combinations")

    System_Ext(bss, "BSS — Billing Support System", "Customer billing, invoicing, payment processing, balance management, revenue assurance")
    System_Ext(oss, "OSS — Operations Support System", "Network provisioning, SIM activation, number portability, service configuration")
    System_Ext(productCatalog, "Product Catalog", "Telecom plans, add-ons, devices, bundles — master product definitions")
    System_Ext(paymentGateway, "Payment Gateway", "Credit card, bank transfer, digital wallet payment processing")
    System_Ext(commChannels, "Communication Channels", "SMS gateway, email service, push notification, WhatsApp Business API")
    System_Ext(regulatory, "Regulatory Authority (ANATEL)", "Telecom compliance filings, number portability database, consumer protection")
    System_Ext(partnerSystems, "Dealer/Partner Systems", "MVNO partners, authorized dealers, franchise management systems")
    System_Ext(dataAnalytics, "Data Analytics Platform", "Big data / ML platform for churn prediction, segmentation models, propensity scoring")

    Rel(salesRep, crm, "Manages customer accounts, processes orders, applies offers")
    Rel(marketingAnalyst, crm, "Defines categories, configures policies, launches campaigns")
    Rel(supportAgent, crm, "Views customer 360, executes retention actions, resolves complaints")
    Rel(channelManager, crm, "Configures partner policies, monitors dealer performance")
    Rel(customer, crm, "Self-service: views plans, accepts offers, manages account, contacts support")
    Rel(complianceOfficer, crm, "Audits data access, reviews consent status, generates compliance reports")

    Rel(crm, bss, "Queries billing data, triggers charges/credits, syncs payment status")
    Rel(crm, oss, "Provisions services, activates SIMs, executes number portability")
    Rel(crm, productCatalog, "Fetches available plans/offers, validates eligibility")
    Rel(crm, paymentGateway, "Processes payments, manages recurring billing, handles refunds")
    Rel(crm, commChannels, "Sends campaign communications, transactional notifications, support messages")
    Rel(crm, regulatory, "Submits compliance reports, queries portability DB, responds to regulatory requests")
    Rel(crm, partnerSystems, "Syncs dealer orders, distributes partner-specific offers, reports commissions")
    Rel(crm, dataAnalytics, "Sends customer events for ML models, receives churn scores and segments")
```

## Key Observations

- **6 actor types** representing internal commercial staff, partners, customers, and compliance roles
- **8 external system integrations** spanning the telecom BSS/OSS stack, payments, communications, regulatory, and analytics
- The CRM is the central commercial hub connecting customer management to telecom operations
- Marketing Business Analyst is the key differentiator — they configure categories and policies without developer involvement
- Customers interact through multiple channels (app, web, store, call center) creating a true omnichannel model
- Regulatory compliance (ANATEL/equivalent) is a first-class concern with direct integration
- The platform bridges commercial intent (marketing policies) with operational execution (BSS/OSS provisioning)
