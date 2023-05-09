use actix_web::web;
use chrono::{Datelike, Timelike, Utc};

use crate::{
    infra::metrics,
    meta::{
        usage::{RequestStats, UsageData, UsageEvent},
        StreamType,
    },
};

use super::logs::json;

pub async fn report_ingest_stats(
    stats: &RequestStats,
    org_id: &str,
    stream_type: StreamType,
    event: UsageEvent,
) {
    let local_stream_type = stream_type.to_string();
    // metrics
    metrics::HTTP_RESPONSE_TIME
        .with_label_values(&[&event.to_string(), "200", org_id, "", &local_stream_type])
        .observe(stats.response_time);
    metrics::HTTP_INCOMING_REQUESTS
        .with_label_values(&[&event.to_string(), "200", org_id, "", &local_stream_type])
        .inc();

    let now = Utc::now();

    let usage = vec![UsageData {
        event,
        day: now.day(),
        hour: now.hour(),
        month: now.month(),
        year: now.year() as i32,
        organization_identifier: org_id.to_owned(),
        request_body: "".to_owned(),
        size: stats.size,
        unit: "bytes".to_owned(),
        user_email: "".to_owned(),
        response_time: stats.response_time,
        stream_type,
    }];
    let json_str = serde_json::to_string(&usage).unwrap();
    let _ = json::ingest(
        "_meta",
        "usage",
        actix_web::web::Bytes::from(json_str),
        web::Data::new(0),
    )
    .await;
}
