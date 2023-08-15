use super::schema::{
    Nodeinfo, NodeinfoMetadata, NodeinfoServices, NodeinfoSoftware, NodeinfoUsage,
    NodeinfoUsageUsers, WebfingerQuery, WellknownNodeinfo, WellknownNodeinfoLink,
    WellknownWebfinger, WellknownWebfingerLink,
};
use crate::{
    misc::{SOFTWARE_NAME, VERSION},
    web::{
        error::{bail_other, map_err_generic, map_err_repository, MxResult},
        extract::RjQuery,
        state::AppState,
    },
};

use axum::{
    extract::{Query, State},
    http::{header::CONTENT_TYPE, StatusCode},
    response::Response,
    Json,
};
use axum_extra::extract::WithRejection;
use monaxia_data::{ap::Acct, config::UserRegistration};
use monaxia_repository::repo::user::UserFind;

pub async fn host_meta(State(state): State<AppState>) -> MxResult<Response<String>> {
    let server_base_url = state.config.cached.server_base_url();
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
    State(state): State<AppState>,
    WithRejection(Query(query), _): RjQuery<WebfingerQuery>,
) -> MxResult<Response<String>> {
    let (config, container) = (state.config, state.container);

    let acct = Acct::parse(&query.resource)
        .map_err(|e| map_err_generic(e, StatusCode::UNPROCESSABLE_ENTITY))?;
    if acct.origin() != config.cached.acct_origin() {
        bail_other(StatusCode::NOT_FOUND, "origin does not match")?;
    }

    let Some(local_user) = container.user
        .find_local_user(UserFind::Username(acct.username()))
        .await
        .map_err(map_err_repository)? else {
        return bail_other(StatusCode::NOT_FOUND, format!("user {} not found", acct.username()));
    };

    let user_url = config
        .cached
        .server_base_url()
        .join(&format!("/users/{}", local_user.id))
        .expect("URL error");
    let data = WellknownWebfinger {
        subject: acct.to_subject(),
        links: vec![WellknownWebfingerLink {
            rel: "self".into(),
            r#type: "application/activity+json".into(),
            href: user_url,
        }],
    };
    let body = serde_json::to_string(&data).expect("JSON error");
    Ok(Response::builder()
        .header(CONTENT_TYPE, "application/jrd+json; charset=UTF-8")
        .body(body)
        .expect("cannot construct"))
}

pub async fn wellknown_nodeinfo(
    State(state): State<AppState>,
) -> MxResult<Json<WellknownNodeinfo>> {
    let nodeinfo_url = state
        .config
        .cached
        .server_base_url()
        .join("/nodeinfo/2.1")
        .expect("URL error");

    Ok(Json(WellknownNodeinfo {
        links: vec![WellknownNodeinfoLink {
            rel: "http://nodeinfo.diaspora.software/ns/schema/2.1".into(),
            href: nodeinfo_url,
        }],
    }))
}

pub async fn nodeinfo(State(state): State<AppState>) -> MxResult<Json<Nodeinfo>> {
    let local_users = state
        .container
        .user
        .local_users_count()
        .await
        .map_err(map_err_repository)?;
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
