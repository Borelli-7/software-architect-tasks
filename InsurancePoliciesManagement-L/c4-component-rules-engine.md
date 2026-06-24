# C4 Component Diagram — Industry Rules Engine

## Description
Internal components of the Industry Rules Engine service — the key architectural differentiator that enables a single platform to serve multiple industries. Business analysts define rules per industry via a low-code interface; the engine evaluates those rules at runtime during policy creation, endorsement, claims, and billing operations.

```mermaid
C4Component
    title Component Diagram — Industry Rules Engine

    Container_Boundary(rules_svc, "Industry Rules Engine Service") {
        Component(rule_api, "Rule Management API", "REST Controller", "CRUD operations on rule sets — create, version, activate, deprecate")
        Component(eval_api, "Rule Evaluation API", "gRPC/REST Controller", "Synchronous rule evaluation endpoint called by Policy, Claims, Billing services")
        Component(rule_builder, "Rule Definition Interface", "Low-Code UI Backend", "Visual rule builder for business analysts — conditions, actions, decision tables")

        Component(adapter_registry, "Industry Adapter Registry", "Plugin Manager", "Discovers and loads industry-specific adapters — coverage types, eligibility, benefit calculations")
        Component(eval_engine, "Rule Evaluation Engine", "Core Engine", "Executes rule sets against policy/claim context using Drools/OPA runtime")
        Component(rule_compiler, "Rule Compiler", "Build Service", "Compiles declarative rule definitions into executable evaluation artifacts")
        Component(sandbox, "Rule Testing Sandbox", "Isolated Runtime", "Executes rules against synthetic data for validation before production deployment")
        Component(version_manager, "Rule Version Manager", "Domain Service", "Manages rule lifecycle: Draft→Testing→Active→Deprecated, rollback support")
        Component(cache_manager, "Rule Cache Manager", "Cache Layer", "Caches compiled rule sets in Redis for fast evaluation — invalidates on version change")

        Component(rule_repo, "Rule Repository", "JPA/EF Repository", "Persists rule definitions, versions, evaluation audit logs")
    }

    Container_Boundary(adapters, "Industry Adapters (Plugins)") {
        Component(finance_adapter, "Finance Industry Adapter", "Plugin", "Banking/financial services: D&O liability, professional indemnity, key person coverage")
        Component(manufacturing_adapter, "Manufacturing Adapter", "Plugin", "Industrial: workplace injury, equipment damage, occupational disease coverage")
        Component(technology_adapter, "Technology Adapter", "Plugin", "Tech companies: cyber liability, IP protection, remote worker coverage")
        Component(engineering_adapter, "Engineering Adapter", "Plugin", "Construction/engineering: project-based coverage, contractors, site liability")
        Component(healthcare_adapter, "Healthcare Adapter", "Plugin", "Medical: malpractice, clinical trials, patient data breach coverage")
    }

    ContainerDb(rulesDb, "Rules DB", "PostgreSQL", "Rule definitions, versions, audit logs")
    Container(cache, "Cache", "Redis", "Compiled rule cache")
    Container(broker, "Message Broker", "Kafka", "Rule change events")

    Rel(rule_api, version_manager, "Manages rule lifecycle")
    Rel(rule_api, rule_compiler, "Triggers compilation")
    Rel(rule_builder, rule_api, "Submits rule definitions")
    Rel(eval_api, eval_engine, "Delegates evaluation")
    Rel(eval_api, cache_manager, "Checks cached rules first")

    Rel(eval_engine, adapter_registry, "Loads industry adapter")
    Rel(adapter_registry, finance_adapter, "Loads")
    Rel(adapter_registry, manufacturing_adapter, "Loads")
    Rel(adapter_registry, technology_adapter, "Loads")
    Rel(adapter_registry, engineering_adapter, "Loads")
    Rel(adapter_registry, healthcare_adapter, "Loads")

    Rel(rule_compiler, sandbox, "Validates compiled rules")
    Rel(version_manager, rule_repo, "Persists versions")
    Rel(version_manager, cache_manager, "Invalidates on publish")
    Rel(cache_manager, cache, "GET/SET compiled rules", "Redis protocol")
    Rel(rule_repo, rulesDb, "SQL queries", "TLS")
    Rel(version_manager, broker, "Publishes RuleActivated, RuleDeprecated", "Kafka/TLS")
```

## Industry Adapter Capabilities

| Industry | Coverage Types | Eligibility Rules | Benefit Calculation |
|----------|---------------|-------------------|---------------------|
| **Finance** | D&O liability, Professional indemnity, Fidelity bond, Key person | Role-based (executives, traders), Regulatory licensing | Tier-based by AUM, bonus structure inclusion |
| **Manufacturing** | Workplace injury, Equipment damage, Occupational disease, Product liability | Job classification (blue/white collar), Shift patterns, Hazard exposure | Risk-weighted by job category, safety record discount |
| **Technology** | Cyber liability, IP protection, Remote worker, E&O | Employment type (FTE, contractor), Remote/hybrid status | Revenue-based, data volume factor, remote work rider |
| **Engineering** | Project-based coverage, Contractor liability, Site safety, Environmental | Project duration, Contractor certification, Site classification | Project value percentage, contractor multiplier |
| **Healthcare** | Malpractice, Clinical trials, Patient data breach, Infectious disease | Medical specialty, Hospital affiliation, Research involvement | Specialty risk tier, claims history factor |

## Rule Definition Lifecycle

```mermaid
stateDiagram-v2
    [*] --> Draft : Business analyst creates rule
    Draft --> Testing : Submitted for validation
    Testing --> Draft : Sandbox tests fail
    Testing --> Active : All tests pass + approval
    Active --> Deprecated : Superseded by new version
    Active --> Active : Minor update (backward compatible)
    Deprecated --> [*] : Retention period expires
```

## Rule Evaluation Flow

```mermaid
sequenceDiagram
    participant Caller as Policy/Claims/Billing Service
    participant API as Rule Evaluation API
    participant Cache as Rule Cache Manager
    participant Engine as Evaluation Engine
    participant Adapter as Industry Adapter
    participant DB as Rules DB

    Caller->>API: EvaluateRules(context, industryId, ruleType)
    API->>Cache: Get compiled rules (industryId + version)
    
    alt Cache hit
        Cache-->>API: Compiled rule set
    else Cache miss
        API->>DB: Fetch active rule set for industry
        DB-->>API: Rule definition
        API->>Cache: Store compiled rules
    end

    API->>Engine: Execute(compiledRules, context)
    Engine->>Adapter: Load industry adapter (industryId)
    Adapter-->>Engine: Industry-specific functions
    Engine->>Engine: Evaluate conditions + actions
    Engine-->>API: RuleResult (pass/fail, adjustments, messages)
    API-->>Caller: EvaluationResponse
```

## Who Defines Rules?

| Role | Responsibility | Access Level |
|------|---------------|--------------|
| **Industry Business Analyst** | Authors rule conditions, decision tables, coverage catalogs per industry | Full CRUD on rules within assigned industry |
| **Senior Business Analyst** | Approves rules for production deployment, cross-industry consistency review | Approve/reject across all industries |
| **Platform Architect** | Defines adapter interfaces, core evaluation engine, plugin contracts | Code-level changes to engine/adapters |
| **QA/Actuary** | Validates rule outcomes against expected actuarial models in sandbox | Read + execute sandbox tests |

## Scaling Considerations

| Scenario | Strategy |
|----------|----------|
| High evaluation throughput (policy creation surge) | Horizontal scaling + Redis-cached compiled rules |
| New industry onboarding | Deploy new adapter plugin, no core service redeploy |
| Rule hot-reload | Version-based cache invalidation, blue-green rule activation |
| Sandbox isolation | Separate container pool for sandbox execution (no production data access) |
