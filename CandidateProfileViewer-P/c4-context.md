# C4 System Context Diagram — Candidate Profile Viewer

## Description
Shows the Candidate Profile Viewer module in context with its external actors and neighboring systems within the executive recruitment agency.

```mermaid
C4Context
    title System Context Diagram — Candidate Profile Viewer

    Person(recruiter, "Recruiter", "Executive recruitment consultant who searches and views candidate LinkedIn profiles")
    Person(admin, "System Administrator", "Manages API credentials, monitors system health, handles configuration")

    System(profileViewer, "Candidate Profile Viewer", "Allows recruiters to search candidates by name and view their LinkedIn profile without logging in to LinkedIn — handles disambiguation and missing profiles")

    System_Ext(linkedinApi, "LinkedIn API", "LinkedIn Talent Solutions REST API — profile search and retrieval endpoints")
    System_Ext(ats, "ATS / CRM System", "Existing Applicant Tracking System — candidate records, job requisitions, recruiter assignments")
    System_Ext(authProvider, "Identity Provider", "OAuth2/OIDC provider for recruiter authentication (Auth0/Keycloak)")

    Rel(recruiter, profileViewer, "Searches candidates by name, views LinkedIn profiles, confirms matches")
    Rel(admin, profileViewer, "Manages API quotas, monitors cache health, configures TTL policies")

    Rel(profileViewer, linkedinApi, "Searches profiles, retrieves profile data", "HTTPS/REST + OAuth2")
    Rel(profileViewer, ats, "Reads candidate records, stores profile links", "REST/JSON")
    Rel(authProvider, profileViewer, "Issues and validates JWT tokens", "OAuth2/OIDC")
```

## Key Observations

- **2 actor types**: recruiters (primary users, ~50–200 per agency) who search and view profiles; administrators who manage API credentials and system configuration
- **1 primary external API**: LinkedIn Talent Solutions API — the core data source with strict rate limits (~100 calls/day per app depending on tier)
- **1 internal system integration**: ATS/CRM providing existing candidate records and storing confirmed LinkedIn profile links
- **1 identity provider**: External OAuth2/OIDC service for recruiter authentication (no LinkedIn login required for recruiters)
- All communication with LinkedIn API uses HTTPS with OAuth2 bearer tokens (agency-level credentials)
- The system boundary clearly separates the agency's internal tools from LinkedIn's external platform
- Missing LinkedIn profiles are handled within the system boundary — no error propagation to external systems
