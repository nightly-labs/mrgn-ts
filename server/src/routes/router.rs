use super::cloud_router::cloud_router;
use crate::{
    handle_error::handle_error,
    http::{
        cloud::cloud_middleware::db_cloud_middleware,
        relay::{
            connect_session::connect_session, drop_sessions::drop_sessions,
            get_pending_request::get_pending_request, get_pending_requests::get_pending_requests,
            get_session_info::get_session_info, get_sessions::get_sessions,
            get_wallets_metadata::get_wallets_metadata, resolve_request::resolve_request,
        },
    },
    sesssion_cleaner::start_cleaning_sessions,
    state::ServerState,
    structs::http_endpoints::HttpEndpoint,
    utils::{get_cors, get_wallets_metadata_vec},
    ws::{
        app_handler::handler::on_new_app_connection,
        client_handler::handler::on_new_client_connection,
    },
};
use axum::{
    error_handling::HandleErrorLayer,
    middleware,
    routing::{get, post},
    Router,
};
use database::db::Db;
use std::{sync::Arc, time::Duration};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;
use tracing_subscriber::EnvFilter;

pub async fn get_router(only_relay_service: bool) -> Router {
    let db = if only_relay_service {
        None
    } else {
        Some(Arc::new(Db::connect_to_the_pool().await))
    };

    let state = ServerState {
        sessions: Default::default(),
        client_to_sessions: Default::default(),
        client_to_sockets: Default::default(),
        wallets_metadata: Arc::new(get_wallets_metadata_vec()),
        session_to_app_map: Default::default(),
        db,
    };

    // Start cleaning outdated sessions
    start_cleaning_sessions(
        &state.sessions,
        &state.client_to_sessions,
        &state.session_to_app_map,
    );
    let cors = get_cors();

    let filter: EnvFilter = "debug,tower_http=trace,hyper=warn"
        .parse()
        .expect("filter should parse");

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let router = if only_relay_service {
        Router::new()
    } else {
        Router::new()
            .nest(
                "/cloud",
                cloud_router(state.clone()).route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    db_cloud_middleware,
                )),
            )
            .with_state(state.clone())
    };

    return router
        .route("/client", get(on_new_client_connection))
        .route("/app", get(on_new_app_connection))
        .route(
            &HttpEndpoint::GetSessionInfo.to_string(),
            post(get_session_info),
        )
        .route(
            &HttpEndpoint::ConnectSession.to_string(),
            post(connect_session),
        )
        .route(&HttpEndpoint::GetSessions.to_string(), post(get_sessions))
        .route(&HttpEndpoint::DropSessions.to_string(), post(drop_sessions))
        .route(
            &HttpEndpoint::GetPendingRequests.to_string(),
            post(get_pending_requests),
        )
        .route(
            &HttpEndpoint::GetPendingRequest.to_string(),
            post(get_pending_request),
        )
        .route(
            &HttpEndpoint::ResolveRequest.to_string(),
            post(resolve_request),
        )
        .route(
            &HttpEndpoint::GetWalletsMetadata.to_string(),
            get(get_wallets_metadata),
        )
        .with_state(state)
        .layer(TraceLayer::new_for_http())
        .layer(
            ServiceBuilder::new()
                .layer(HandleErrorLayer::new(handle_error))
                .timeout(Duration::from_secs(10)),
        )
        .layer(cors);
}
