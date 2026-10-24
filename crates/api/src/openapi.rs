//! OpenAPI 3.1 discovery document for the supported HTTP surface.
//!
//! Bodies are intentionally generic objects: field-level contracts live in the
//! event schemas and `docs/ARCHITECTURE.md`. This document is for discovery,
//! client generation, and tooling.

use axum::Json;
use serde_json::{json, Map, Value};

fn op(id: &str, tag: &str, summary: &str, with_body: bool) -> Value {
    let mut operation = Map::new();
    operation.insert("operationId".into(), json!(id));
    operation.insert("summary".into(), json!(summary));
    operation.insert("tags".into(), json!([tag]));
    operation.insert(
        "responses".into(),
        json!({
            "200": {"description": "Successful response",
                    "content": {"application/json": {"schema": {"type": "object"}}}},
            "400": {"description": "Invalid request"},
            "401": {"description": "Authentication required"},
            "403": {"description": "Forbidden"}
        }),
    );
    if with_body {
        operation.insert(
            "requestBody".into(),
            json!({"required": true,
                   "content": {"application/json": {"schema": {"type": "object"}}}}),
        );
    }
    Value::Object(operation)
}

fn paths() -> Value {
    let mut paths = Map::new();
    let get = |id: &str, tag: &str, summary: &str| json!({"get": op(id, tag, summary, false)});
    let post = |id: &str, tag: &str, summary: &str, body: bool| json!({"post": op(id, tag, summary, body)});

    paths.insert(
        "/healthz".into(),
        get("healthz", "health", "Liveness probe"),
    );
    paths.insert(
        "/openapi.json".into(),
        get("openapi", "meta", "This OpenAPI document"),
    );
    for (path, id, tag, summary) in [
        ("/v1/track", "track", "tracking", "Track an event"),
        ("/v1/identify", "identify", "tracking", "Identify a user"),
        ("/v1/page", "page", "tracking", "Track a page view"),
        ("/v1/screen", "screen", "tracking", "Track a screen view"),
        (
            "/v1/group",
            "group",
            "tracking",
            "Associate a user with a group",
        ),
        ("/v1/alias", "alias", "tracking", "Alias two identifiers"),
        ("/v1/batch", "batch", "tracking", "Track a batch of events"),
        ("/v1/squid", "squid", "tracking", "Squid-CRM webhook"),
        (
            "/events",
            "post_event",
            "events",
            "Ingest a canonical event",
        ),
        (
            "/schemas",
            "schemas",
            "schemas",
            "List or create event schemas",
        ),
        ("/mixins", "mixins", "mixins", "List or create mixins"),
        ("/namespaces", "namespaces", "namespaces", "List namespaces"),
        (
            "/audiences",
            "audiences",
            "audiences",
            "List or create audiences",
        ),
        (
            "/destinations",
            "destinations",
            "activation",
            "List or create destinations",
        ),
        (
            "/dead-letter-events",
            "dead_letter",
            "events",
            "List dead-letter events",
        ),
        (
            "/policies",
            "policies",
            "governance",
            "List or create policies",
        ),
        ("/consent", "record_consent", "governance", "Record consent"),
        (
            "/policy-simulate",
            "policy_simulate",
            "governance",
            "Simulate a policy",
        ),
        (
            "/channels",
            "channels",
            "channels",
            "List or create channels",
        ),
        (
            "/templates",
            "templates",
            "templates",
            "List or create templates",
        ),
        (
            "/journeys",
            "journeys",
            "journeys",
            "List or create journeys",
        ),
        (
            "/contact-policies",
            "contact_policies",
            "journeys",
            "List or create contact policies",
        ),
    ] {
        let body = !matches!(
            path,
            "/events"
                | "/schemas"
                | "/mixins"
                | "/audiences"
                | "/destinations"
                | "/policies"
                | "/consent"
                | "/policy-simulate"
                | "/channels"
                | "/templates"
                | "/journeys"
                | "/contact-policies"
        );
        let method = if path.starts_with("/v1/")
            || path == "/events"
            || path == "/consent"
            || path == "/policy-simulate"
        {
            post(id, tag, summary, true)
        } else {
            json!({
                "get": op(&format!("{id}_list"), tag, summary, false),
                "post": op(&format!("{id}_create"), tag, summary, true)
            })
        };
        let _ = body;
        paths.insert(path.into(), method);
    }
    for (path, id, tag, summary) in [
        ("/events/{id}", "get_event", "events", "Fetch an event"),
        (
            "/profiles/{id}",
            "get_profile",
            "profiles",
            "Fetch a profile",
        ),
        (
            "/profiles/{id}/events",
            "profile_events",
            "profiles",
            "List profile events",
        ),
        (
            "/profiles/{id}/identity",
            "profile_identity",
            "profiles",
            "Fetch identity links",
        ),
        (
            "/profiles/{id}/merge-suggestions",
            "merge_suggestions",
            "profiles",
            "List merge suggestions",
        ),
        (
            "/profiles/{id}/audiences",
            "profile_audiences",
            "profiles",
            "List profile audiences",
        ),
        (
            "/profiles/{id}/consent",
            "profile_consent",
            "governance",
            "Fetch profile consent",
        ),
        (
            "/profiles/{id}/journeys",
            "profile_journeys",
            "journeys",
            "List profile journeys",
        ),
        (
            "/schemas/{name}/{version}",
            "get_schema",
            "schemas",
            "Fetch a schema",
        ),
        (
            "/mixins/{namespace}/{name}/{version}",
            "get_mixin",
            "mixins",
            "Fetch a mixin",
        ),
        (
            "/audiences/{id}",
            "get_audience",
            "audiences",
            "Fetch an audience",
        ),
        (
            "/audiences/{id}/members",
            "audience_members",
            "audiences",
            "List audience members",
        ),
        (
            "/journeys/{id}",
            "get_journey",
            "journeys",
            "Fetch a journey",
        ),
        (
            "/journeys/{id}/runs",
            "journey_runs",
            "journeys",
            "List journey runs",
        ),
        (
            "/journey-runs/{id}/events",
            "run_events",
            "journeys",
            "List run events",
        ),
    ] {
        paths.insert(path.into(), get(id, tag, summary));
    }
    for (path, id, tag, summary) in [
        (
            "/profiles/{id}/merge",
            "merge_profile",
            "profiles",
            "Merge a profile",
        ),
        (
            "/profiles/{id}/split",
            "split_profile",
            "profiles",
            "Split a profile",
        ),
        (
            "/audiences/{id}/activate",
            "activate_audience",
            "activation",
            "Activate an audience",
        ),
        (
            "/audiences/{id}/activate-via-reverse-etl",
            "activate_reverse_etl",
            "activation",
            "Activate via reverse ETL",
        ),
        (
            "/journeys/{id}/start",
            "start_journey",
            "journeys",
            "Start a journey",
        ),
    ] {
        paths.insert(path.into(), post(id, tag, summary, true));
    }
    Value::Object(paths)
}

pub fn document() -> Value {
    json!({
        "openapi": "3.1.0",
        "info": {
            "title": "PyCustomerHub API",
            "version": env!("CARGO_PKG_VERSION"),
            "description": "Event-driven Customer Data & Engagement Platform. \
                Bodies are generic objects; field-level contracts live in the \
                event schemas and docs/ARCHITECTURE.md."
        },
        "servers": [{"url": "/", "description": "Same origin"}],
        "components": {
            "securitySchemes": {
                "bearerAuth": {"type": "http", "scheme": "bearer"},
                "writeKey": {"type": "apiKey", "in": "header", "name": "X-Write-Key"}
            }
        },
        "paths": paths()
    })
}

pub async fn spec() -> Json<Value> {
    Json(document())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn document_covers_core_surfaces() {
        let doc = document();
        assert_eq!(doc["openapi"], "3.1.0");
        let paths = doc["paths"].as_object().unwrap();
        for path in [
            "/v1/track",
            "/events",
            "/profiles/{id}",
            "/audiences",
            "/journeys",
            "/openapi.json",
        ] {
            assert!(paths.contains_key(path), "missing {path}");
        }
    }

    #[test]
    fn every_path_has_operations() {
        let doc = document();
        for (path, item) in doc["paths"].as_object().unwrap() {
            assert!(
                item.as_object().is_some_and(|ops| !ops.is_empty()),
                "{path} has no operations"
            );
        }
    }
}
