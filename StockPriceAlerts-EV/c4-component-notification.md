# C4 Component Diagram — Notification Service

## Description
Internal components of the Notification Service, showing how triggered alerts are consumed, routed to the appropriate delivery channel(s), dispatched, and tracked for delivery status.

```mermaid
C4Component
    title Component Diagram — Notification Service

    Container_Boundary(notificationSvc, "Notification Service") {
        Component(alertConsumer, "Alert Consumer", "Kafka Consumer", "Consumes AlertTriggered events from Kafka; deserializes and validates payload")
        Component(channelRouter, "Channel Router", "Routing Logic", "Determines delivery channel(s) per user preferences: email, SMS, push, or combination")
        Component(templateEngine, "Template Engine", "Mustache/Thymeleaf", "Renders notification content from alert data using channel-specific templates")
        Component(emailSender, "Email Sender", "SMTP Client", "Formats and dispatches alert emails via external SMTP gateway")
        Component(smsSender, "SMS Sender", "HTTP Client", "Sends alert SMS via Twilio API with character-limited formatting")
        Component(pushSender, "Push Sender", "HTTP Client", "Sends mobile push notifications via FCM (Android) and APNs (iOS)")
        Component(deliveryTracker, "Delivery Tracker", "Event Logger", "Records delivery status (sent, delivered, failed, bounced) and publishes DeliveryStatus events")
        Component(retryHandler, "Retry Handler", "Scheduler", "Retries failed deliveries with exponential backoff; routes to Dead Letter Queue after max attempts")
    }

    Container(broker, "Message Broker", "Kafka", "Event streaming")
    ContainerDb(userDb, "User/Preferences DB", "PostgreSQL", "User channel preferences")
    System_Ext(emailGateway, "Email Gateway", "SMTP relay")
    System_Ext(smsGateway, "SMS Gateway", "Twilio")
    System_Ext(pushService, "Push Service", "FCM/APNs")

    Rel(broker, alertConsumer, "AlertTriggered events", "Kafka/TLS")
    Rel(alertConsumer, channelRouter, "Alert payload + userId")
    Rel(channelRouter, userDb, "Fetch user preferences", "SQL/TLS")
    Rel(channelRouter, templateEngine, "Alert + channel type")
    
    Rel(templateEngine, emailSender, "Rendered email content")
    Rel(templateEngine, smsSender, "Rendered SMS text")
    Rel(templateEngine, pushSender, "Rendered push payload")
    
    Rel(emailSender, emailGateway, "Send email", "SMTP/TLS")
    Rel(smsSender, smsGateway, "Send SMS", "HTTPS")
    Rel(pushSender, pushService, "Send push", "HTTPS")
    
    Rel(emailSender, deliveryTracker, "Delivery result")
    Rel(smsSender, deliveryTracker, "Delivery result")
    Rel(pushSender, deliveryTracker, "Delivery result")
    
    Rel(deliveryTracker, broker, "DeliveryStatus events", "Kafka/TLS")
    Rel(deliveryTracker, retryHandler, "Failed deliveries")
    Rel(retryHandler, channelRouter, "Retry dispatch")
```

## Delivery Guarantees

| Aspect | Design |
|--------|--------|
| **Semantics** | At-least-once delivery — idempotency key per alert+user prevents duplicate notifications |
| **Deduplication** | SHA-256 hash of (ruleId + userId + triggerTimestamp) stored in Redis with TTL |
| **Retry policy** | Exponential backoff: 1s → 5s → 30s → 5min; max 5 attempts |
| **Dead Letter Queue** | After max retries, message routed to DLQ for manual investigation |
| **Quiet hours** | Channel Router respects per-user quiet hours; defers delivery to next active window |
| **Rate limiting** | Max 10 notifications/minute per user across all channels; excess queued |
| **Circuit breaker** | Per-channel circuit breaker (Resilience4j pattern); opens after 5 consecutive failures |

## Notification Templates

| Channel | Format | Constraints |
|---------|--------|-------------|
| Email | HTML + plain text fallback | Subject: "[Alert] {symbol} {condition}"; body includes price, timestamp, link to portfolio |
| SMS | Plain text | Max 160 characters: "{symbol} hit €{price} ({condition}). View: {shortUrl}" |
| Push | Title + body + deep link | Title: "{symbol} Alert"; body: "Price {direction} €{threshold}"; tap opens app |

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `NotificationSent` | Successful dispatch to any channel | Alert History (audit), Analytics |
| `NotificationFailed` | All retry attempts exhausted | Admin Monitoring, DLQ processor |
| `NotificationDeferred` | Quiet hours active; delivery postponed | Scheduler (re-enqueue at window open) |
