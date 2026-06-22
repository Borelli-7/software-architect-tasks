# C4 System Context Diagram — Stock Price Alerts

## Description
Shows the Stock Price Alert system in context with its external actors and neighboring systems.

```mermaid
C4Context
    title System Context Diagram — Stock Price Alerts

    Person(customer, "Retail Customer", "Brokerage client who defines price alerts and receives notifications")
    Person(admin, "System Administrator", "Manages system configuration, monitors health, handles escalations")

    System(alertSystem, "Stock Price Alert System", "Monitors real-time stock prices and notifies customers when their defined conditions are met — supports thousands of stocks and millions of alert rules")

    System_Ext(marketData, "Market Data Providers", "Bloomberg, Reuters, exchange feeds — real-time and delayed price data")
    System_Ext(emailGateway, "Email Gateway", "SMTP relay for transactional email delivery")
    System_Ext(smsGateway, "SMS Gateway", "Twilio or equivalent — SMS delivery to customer phones")
    System_Ext(pushService, "Push Notification Service", "FCM (Android) / APNs (iOS) — mobile push delivery")
    System_Ext(brokerage, "Brokerage Trading Platform", "Customer accounts, portfolio data, subscription tiers")

    Rel(customer, alertSystem, "Defines alerts, views history, manages preferences")
    Rel(admin, alertSystem, "Monitors pipeline health, manages configuration")

    Rel(marketData, alertSystem, "Streams real-time price updates", "WebSocket/FIX")
    Rel(alertSystem, emailGateway, "Sends alert emails", "SMTP/TLS")
    Rel(alertSystem, smsGateway, "Sends alert SMS", "HTTPS/REST")
    Rel(alertSystem, pushService, "Sends push notifications", "HTTPS/REST")
    Rel(alertSystem, brokerage, "Validates customer, fetches portfolio", "REST/gRPC")
```

## Key Observations

- **2 actor types**: retail customers (thousands) who define and receive alerts; administrators who maintain the system
- **1 inbound data source**: Market Data Providers streaming prices for thousands of stocks in real-time
- **3 outbound notification channels**: email, SMS, and push — each with distinct delivery characteristics and SLAs
- **1 internal system integration**: Brokerage Trading Platform for customer validation and subscription enforcement
- All market data enters the system boundary via secure streaming protocols (WebSocket/FIX)
- All notification dispatch crosses the boundary via encrypted channels (TLS/HTTPS)
