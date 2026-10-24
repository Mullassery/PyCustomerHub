# Competitive intelligence in roadmap and upgrade work

**Policy date:** 2026-10-19

Competitive findings are an input to product decisions, not a one-off report.
Every pending roadmap item, new feature, and routine dependency upgrade should
check the repository's [`AUDIT_FINDINGS.md`](AUDIT_FINDINGS.md) before work
starts, and record the decision in the PR or changelog.

## Required check before building

1. **Read `AUDIT_FINDINGS.md`.** Find the relevant competitor row and the
   ranked next actions. If the finding is stale, update it with a dated source.
2. **Name the competitor and the user problem.** A feature is not justified by
   "competitor has it". State the workflow it unlocks, who needs it, and how
   often. Mark unverified demand as a hypothesis.
3. **Prefer reuse over new code.** Check whether an existing Mullassery repo
   already provides the capability (see `BUNDLES.md` in the profile repo and
   the bundle role file in this repo, when present). Typical reuse:
   - crawling/fetching/robots/SSRF → **MudFish**
   - architecture maps / PR diffs → **CodeAtlas**
   - data-quality contracts / validation → **StatGuardian**
   - token counting / cost tables → **PyTokenCalc**
   - dependency graph / advisories / SBOM → **PyDeepMesh**
   - segmentation / CLV / churn → **ClusterAudienceKit**
   - activation / lineage / PII → **PyReverseETL**
   - fleet state sync → **SyncFleet** (only after conflict resolution)
4. **Choose one disposition:** build, integrate, defer, or reject. Write it
   down. A rejection with a reason is a successful outcome.
5. **For routine upgrades**, check whether the upgrade closes a known
   competitor gap or breaks a differentiated capability. Run the tests that
   cover the affected path; do not merge a green build that silently changes a
   user-visible contract.

## PR / changelog checklist

- [ ] Competitor and source URL/date recorded (or "no direct competitor").
- [ ] User impact and frequency stated; unverified demand labelled.
- [ ] Reuse check done against sibling repos/bundles.
- [ ] Failure mode and rollback considered (especially auth, PII, cost).
- [ ] Docs/roadmap status updated honestly.
- [ ] Tests or a manual verification command included.

## Review cadence

- Refresh the competitor row when a release changes the capability.
- Re-check the full `AUDIT_FINDINGS.md` at least quarterly and before a major
  release.
- If a finding is wrong or stale, correct it in the same PR that relies on it.

## Non-goals

- Copying features for parity.
- Adding a dependency just because a competitor uses it.
- Treating star counts or download counts as proof of user need.

## OSS gap analysis — 2026-10-24

**What matters most here:** destination breadth and warehouse delivery. Four destinations and no Snowflake/BigQuery/S3 loader mean teams still need another activation tool; broaden the catalog and treat the warehouse as a first-class destination.

Scope: Compared PyCustomerHub's implemented collect → identify → profile → audience → activate → journey path against three self-hostable event/CDP platforms users evaluate for the same job. Assessed destination breadth, identity handling, warehouse delivery, consent/governance, deployment and operating evidence using only this repo's architecture doc plus competitors' READMEs and GitHub metadata.

### Comparable OSS projects
| Project | GitHub | License | Last activity | Why it is the comparison |
| --- | --- | --- | --- | --- |
| RudderStack | https://github.com/rudderlabs/rudder-server | Elastic License 2.0 | Pushed 2026-10-23 | Segment-compatible CDP with event collection, warehouse-first delivery, transformations and a large destination library — the closest commercial OSS analogue |
| Jitsu | https://github.com/jitsucom/jitsu | MIT | Pushed 2026-10-23 | Self-hostable Segment alternative with HTTP/SDK ingestion, identity graph, profile builder, functions and warehouse/SaaS destinations |
| Snowplow | https://github.com/snowplow/snowplow | Limited Use License since 2024-01-08; older releases Apache-2.0 | Pushed 2026-10-19 | Event-data infrastructure with schema validation, enrichment and warehouse/lake/stream loaders; the rigor benchmark for collection |

RudderStack and Jitsu are comparable in scope and both were pushed within the last two days. Snowplow is narrower on activation but stronger on schema-validated event quality; its 2024 license change is material for users choosing an OSS CDP. PostHog was reviewed as a hint but excluded from the table: its centre of gravity is product analytics, replay, flags and experiments rather than profile/audience/activation.

### Capability comparison
| Capability | Ours | RudderStack | Jitsu | Snowplow | Gap |
| --- | --- | --- | --- | --- | --- |
| HTTP event ingestion | Yes | Yes | Yes | Yes | Parity |
| Client SDK breadth | Partial | Yes | Yes | Yes | Android unverified; no server SDKs |
| Schema validation | Yes | Partial | Partial | Yes | Parity with Snowplow; stronger than function-only pipelines |
| Deterministic identity resolution | Yes | Partial | Partial | Partial | Provenance/audit is ours |
| Probabilistic identity | No | Partial | Partial | Partial | Suggestions only, never auto-merge |
| Warehouse/lake loaders | No | Yes | Yes | Yes | We lack it |
| Destination breadth (SaaS/CRM/ads) | Partial | Yes | Yes | Partial | 3 profile destinations + bulk |
| User-defined transformations | No | Yes | Yes | Yes | We lack it |
| Consent ledger + label policy | Yes | Partial | No | Partial | Ours is explicit and per-profile |
| Durable journey orchestration | Yes | No | Partial | No | Ours only in this set |
| Self-hosted without vendor account | Yes | Yes | Yes | Yes | Parity |
| Production-scale evidence | No | Yes | Yes | Yes | 156 events/sec single node |

"Partial" identity resolution for competitors is deliberate: RudderStack and Jitsu expose identity/profile concepts, and Snowplow's identity stitching is typically a warehouse/dbt modelling step, not an append-only identity graph with field-level provenance. "Partial" transformations for RudderStack reflects that transformations exist but are not the same product surface as Jitsu Functions.

### Gaps that matter (ranked)
1. **Destination breadth is the adoption blocker** — `docs/ARCHITECTURE.md` lists only webhook, HubSpot contacts, Klaviyo profiles and a PyReverseETL bulk path; the roadmap's first item is "add 5-10 production-tested destinations". RudderStack and Jitsu ship broad catalogs, so most real activation workflows (Braze, Salesforce, ad-platform CAPI) cannot be served today. Hurts every marketing-ops buyer. Effort L.
2. **No warehouse/lake delivery path** — competitors treat the warehouse as a first-class destination; PyCustomerHub has no Snowflake/BigQuery/Redshift/S3 loader. Teams that want profiles in their own warehouse must run another tool, which removes the main reason to adopt a CDP. Effort L.
3. **Operating evidence is far below competitors** — README claims ~156 events/sec sequential on one connection; `ROADMAP_HONEST.md` confirms there are no live Postgres/Kafka integration tests, no load test, no independent security review, and the edge gateway is undeployed. RudderStack/Jitsu/Snowplow all publish production-scale claims. Hurts platform teams doing due diligence. Effort L.
4. **No user-defined transformations or PII masking before delivery** — every competitor offers a transformation layer (Jitsu Functions, RudderStack transformations, Snowplow enrichments). PyCustomerHub's mapping is fixed by schemas/mixins and connector adapters, so field-level redaction or enrichment requires code changes. Effort M.
5. **Audience semantics are weaker than they look** — no sequence conditions (`A then B then not C`), membership changes are not republished as `customer.entered_audience` events, and a time-window condition only re-evaluates when that profile receives another event. Users migrating from mature CDPs will hit stale exits. Effort M.
6. **Governance is ahead but incomplete** — consent purpose mapping is fixed in code, schema field labels are stored but not enforced, and the PyReverseETL activation path skips field-level label policy. This is a "weaker version" of an otherwise differentiated feature, not an absent one. Effort M.

### Where we are differentiated
- Per-profile governed activation returns explicit `blocked` reasons from label policy, destination capability and consent, and the policy simulator reuses the real evaluation primitives rather than approximating them.
- Field-level provenance plus an append-only identity audit trail and explicit merge/split is more transparent than the event-level pipelines compared here.
- Durable journey runs with idempotent sends (`UNIQUE(run_id, node_id)`) and contact-policy suppression are not offered by RudderStack, Jitsu or Snowplow as part of the same self-hosted package.
- Apache-2.0 with no external accounts, versus RudderStack's ELv2 and Snowplow's 2024 Limited Use License, is a real procurement advantage.

### Disposition
- **Build:** verified destinations (start with Braze, Salesforce, ad-platform CAPI), a warehouse loader, and a transformation/PII-masking step — in that order.
- **Integrate/reuse:** keep PyReverseETL for bulk table-to-destination sync rather than reimplementing it; reuse sibling validation/lineage work where possible.
- **Defer/reject:** sequence audiences, experimentation, AI decisioning and enterprise RBAC stay deferred until connector breadth and operating evidence exist; do not chase RudderStack's full catalog or Snowplow's enrichment library.

### Sources
- https://github.com/rudderlabs/rudder-server (ELv2 license, README, pushed 2026-10-23, accessed 2026-10-24)
- https://github.com/jitsucom/jitsu (MIT license, README identity graph/profile builder, pushed 2026-10-23, accessed 2026-10-24)
- https://github.com/snowplow/snowplow (Limited Use License notice dated 2024-01-08, README, pushed 2026-10-19, accessed 2026-10-24)
- Repo files inspected: `README.md`, `COMPETITIVE_INTELLIGENCE.md`, `AUDIT_FINDINGS.md`, `docs/ARCHITECTURE.md`, `ROADMAP_HONEST.md`, `Cargo.toml`
