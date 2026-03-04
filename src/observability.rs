use std::{
    net::SocketAddr,
    sync::atomic::{AtomicI64, Ordering},
    time::Instant,
};

use anyhow::{Context, Result};
use metrics::{counter, gauge, histogram};
use metrics_exporter_prometheus::PrometheusBuilder;

use crate::config::ObservabilityConfig;

static LAST_SUCCESSFUL_POLL_TS: AtomicI64 = AtomicI64::new(0);

pub fn init_metrics(cfg: &ObservabilityConfig) -> Result<()> {
    if !cfg.metrics_enabled {
        return Ok(());
    }

    let bind = cfg
        .metrics_bind
        .parse::<SocketAddr>()
        .with_context(|| format!("invalid observability.metrics_bind: {}", cfg.metrics_bind))?;

    PrometheusBuilder::new()
        .with_http_listener(bind)
        .install()
        .context("failed to install prometheus recorder/exporter")?;

    tracing::info!(bind = %bind, "prometheus metrics exporter enabled");
    Ok(())
}

pub fn now() -> Instant {
    Instant::now()
}

pub fn observe_stage_latency(stage: &'static str, start: Instant) {
    histogram!(
        "gov_agent_stage_latency_seconds",
        "stage" => stage,
    )
    .record(start.elapsed().as_secs_f64());
}

pub fn incr_proposals_discovered(count: usize) {
    if count > 0 {
        counter!("gov_agent_proposals_discovered_total").increment(count as u64);
    }
}

pub fn incr_proposals_processed() {
    counter!("gov_agent_proposals_processed_total").increment(1);
}

pub fn incr_proposals_failed(stage: &'static str) {
    counter!(
        "gov_agent_proposals_failed_total",
        "stage" => stage,
    )
    .increment(1);
}

pub fn record_vote_submit(success: bool) {
    let status = if success { "success" } else { "failure" };
    counter!(
        "gov_agent_vote_submit_total",
        "status" => status,
    )
    .increment(1);
}

pub fn record_provider_error(provider: &'static str, operation: &'static str) {
    counter!(
        "gov_agent_provider_errors_total",
        "provider" => provider,
        "operation" => operation,
    )
    .increment(1);
}

pub fn record_poll_attempt() {
    let now = chrono::Utc::now().timestamp();
    gauge!("gov_agent_last_poll_attempt_timestamp_seconds").set(now as f64);

    let last_success = LAST_SUCCESSFUL_POLL_TS.load(Ordering::Relaxed);
    if last_success > 0 {
        let staleness = (now - last_success).max(0);
        gauge!("gov_agent_listener_staleness_seconds").set(staleness as f64);
    }
}

pub fn record_poll_success() {
    let now = chrono::Utc::now().timestamp();
    LAST_SUCCESSFUL_POLL_TS.store(now, Ordering::Relaxed);
    gauge!("gov_agent_last_successful_poll_timestamp_seconds").set(now as f64);
    gauge!("gov_agent_listener_staleness_seconds").set(0.0);
}

pub fn record_last_processed_proposal_timestamp() {
    let now = chrono::Utc::now().timestamp();
    gauge!("gov_agent_last_processed_proposal_timestamp_seconds").set(now as f64);
}
