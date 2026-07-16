# C4 Component Diagram — Profile Search Service

## Description
Internal components of the Profile Search microservice, showing how a recruiter's name query is processed through parsing, local index lookup, LinkedIn delegation, and disambiguation before returning results.

```mermaid
C4Component
    title Component Diagram — Profile Search Service

    Container_Boundary(profileSearchSvc, "Profile Search Service") {
        Component(queryParser, "Query Parser", "Input Handler", "Parses recruiter search input — extracts name components, optional filters (company, title, location)")
        Component(localIndexLookup, "Local Index Lookup", "Elasticsearch Client", "Searches internal candidate index for previously linked profiles matching the query")
        Component(linkedinDelegator, "LinkedIn Delegator", "HTTP Client", "Forwards search to LinkedIn Integration Service when local index has no confirmed match")
        Component(resultAggregator, "Result Aggregator", "Merger/Ranker", "Combines local matches and LinkedIn results; ranks by relevance (exact name > fuzzy > partial)")
        Component(disambiguationEngine, "Disambiguation Engine", "Ranking Logic", "Presents multiple matches with preview data for recruiter selection; applies scoring heuristics")
        Component(linkConfirmer, "Link Confirmer", "Persistence Handler", "Stores confirmed candidate-LinkedIn link in DB when recruiter selects the correct match")
    }

    Container(searchIndex, "Candidate Search Index", "Elasticsearch")
    Container(candidateDb, "Candidate Index DB", "PostgreSQL")
    Container(linkedinSvc, "LinkedIn Integration Service", "Node.js")
    Container(cacheSvc, "Profile Cache Service", "Node.js")
    Container(apiGateway, "API Gateway", "NGINX")

    Rel(apiGateway, queryParser, "Search request (name, filters)")
    Rel(queryParser, localIndexLookup, "Parsed query tokens")
    Rel(localIndexLookup, searchIndex, "Fuzzy name search", "REST/JSON")
    Rel(localIndexLookup, resultAggregator, "Local matches (0..N)")
    Rel(queryParser, linkedinDelegator, "Parsed query (when local miss)")
    Rel(linkedinDelegator, cacheSvc, "Check cache first", "HTTPS")
    Rel(linkedinDelegator, linkedinSvc, "Search LinkedIn (cache miss)", "HTTPS")
    Rel(linkedinDelegator, resultAggregator, "LinkedIn matches (0..N)")
    Rel(resultAggregator, disambiguationEngine, "Merged ranked results")
    Rel(disambiguationEngine, apiGateway, "Disambiguation response (ranked list or single match)")
    Rel(disambiguationEngine, linkConfirmer, "Recruiter confirms selection")
    Rel(linkConfirmer, candidateDb, "INSERT candidate_linkedin_link", "SQL/TLS")
```

## Domain Events Published

| Event | Trigger | Consumers |
|-------|---------|-----------|
| `CandidateSearched` | Recruiter submits a name search | Analytics, Audit Log |
| `ProfileLinkConfirmed` | Recruiter selects correct LinkedIn match | Candidate Index, ATS Sync |
| `NoProfileFound` | LinkedIn returns zero results for a query | Candidate Index (flag as "no LinkedIn") |
| `DisambiguationPresented` | Multiple matches returned to recruiter | Analytics (track disambiguation rate) |
| `SearchFromCache` | Result served from cache without LinkedIn call | Monitoring (cache hit tracking) |

## Data Flow Characteristics

| Metric | Value |
|--------|-------|
| Inbound throughput | ~10–50 searches/minute (agency-wide, business hours) |
| Local index size | 10,000–100,000 candidates (agency's historical database) |
| Elasticsearch response | < 50ms (fuzzy name match) |
| LinkedIn API delegation | < 2s (when cache miss, subject to LinkedIn latency) |
| Disambiguation rate | ~30% of searches (common names yield multiple results) |
| Link confirmation rate | ~85% (recruiters find correct profile on first search) |
