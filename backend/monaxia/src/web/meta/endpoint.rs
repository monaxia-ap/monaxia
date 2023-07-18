use super::schema::{
    Nodeinfo, NodeinfoMetadata, NodeinfoServices, NodeinfoSoftware, NodeinfoUsage,
    NodeinfoUsageUsers, WebfingerQuery, WellknownNodeinfo, WellknownNodeinfoLink,
    WellknownWebfinger,
};
use crate::{
    config::UserRegistration,
    constant::{SOFTWARE_NAME, VERSION},
    web::{
        error::{bail_other, map_err_anyhow, MxResult},
        extract::RjQuery,
        state::AppState,
    },
};

use anyhow::Context;
use axum::{
    extract::{Query, State},
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::WithRejection;

pub async fn host_meta(State(state): State<AppState>) -> MxResult<Response<String>> {
    let server_base_url = state
        .config
        .server
        .server_base_url()
        .map_err(map_err_anyhow)?;
    let response = format!(
        r#"<?xml version="1.0"?>
<XRD xmlns="http://docs.oasis-open.org/ns/xri/xrd-1.0">
    <Link rel="lrdd" type="application/xrd+xml" template="{server_base_url}/.well-known/webfinger?resource={{uri}}" />
</XRD>
"#
    );
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/xrd+xml")
        .body(response)
        .expect("cannot construct"))
}

pub async fn wellknown_webfinger(
    WithRejection(Query(query), _): RjQuery<WebfingerQuery>,
) -> MxResult<Json<WellknownWebfinger>> {
    bail_other(
        StatusCode::NOT_FOUND,
        format!("user {} not found", query.resource),
    )
}

pub async fn wellknown_nodeinfo(
    State(state): State<AppState>,
) -> MxResult<Json<WellknownNodeinfo>> {
    let nodeinfo_url = state
        .config
        .server
        .server_base_url()
        .and_then(|bu| bu.join("/nodeinfo/2.1").context("URL error"))
        .map_err(map_err_anyhow)?;

    Ok(Json(WellknownNodeinfo {
        links: vec![WellknownNodeinfoLink {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".into(),
            href: nodeinfo_url,
        }],
    }))
}

pub async fn nodeinfo(State(state): State<AppState>) -> MxResult<Json<Nodeinfo>> {
    let local_users = 0;
    let open_registrations = state.config.user.registration == UserRegistration::Open;
    Ok(Json(Nodeinfo {
        version: "2.1".into(),
        software: NodeinfoSoftware {
            name: SOFTWARE_NAME.into(),
            version: VERSION.into(),
        },
        protocols: vec!["activitypub".into()],
        services: NodeinfoServices {
            inbound: vec![],
            outbound: vec![],
        },
        open_registrations,
        usage: NodeinfoUsage {
            users: NodeinfoUsageUsers { total: local_users },
        },
        metadata: NodeinfoMetadata {},
    }))
}
