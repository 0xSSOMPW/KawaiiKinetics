mod error;
mod handler;

use axum::{routing::get, Extension, Router};
use handler::HiAnime;
use shuttle_runtime::SecretStore;
use std::sync::Arc;
use tokio::sync::Mutex;

use aniscraper::{env::SecretConfig, hianime::HiAnimeRust};

#[shuttle_runtime::main]
async fn axum(#[shuttle_runtime::Secrets] secret_store: SecretStore) -> shuttle_axum::ShuttleAxum {
    let secret_config = SecretConfig {
        max_retries_attempts: secret_store.get("MAX_RETRIES_ATTEMPTS").unwrap_or_default(),
        reqwest_error_webhook: secret_store
            .get("REQWEST_ERROR_WEBHOOK")
            .unwrap_or_default(),
        no_proxies_available_error_webhook: secret_store
            .get("NO_PROXIES_AVAILABLE_ERROR_WEBHOOK")
            .unwrap_or_default(),
        failed_to_fetch_after_retries_error_webhook: secret_store
            .get("FAILED_TO_FETCH_AFTER_RETRIES_ERROR_WEBHOOK")
            .unwrap_or_default(),
        utils_error_webhook: secret_store.get("UTILS_ERROR_WEBHOOK").unwrap_or_default(),
        unknown_error_webhook: secret_store
            .get("UNKNOWN_ERROR_WEBHOOK")
            .unwrap_or_default(),
        http_url: secret_store.get("HTTP_URL").unwrap_or_default(),
        sock4_url: secret_store.get("SOCK4_URL").unwrap_or_default(),
        sock5_url: secret_store.get("SOCK5_URL").unwrap_or_default(),
        hianime_domains: secret_store.get("HIANIME_DOMAINS").unwrap_or_default(),
        user_agent_header: secret_store.get("USER_AGENT_HEADER").unwrap_or_default(),
        accept_encoding_header: secret_store
            .get("ACCEPT_ENCODING_HEADER")
            .unwrap_or_default(),
        accept_header: secret_store.get("ACCEPT_HEADER").unwrap_or_default(),
    };

    let hianime_cache = Arc::new(Mutex::new(HiAnime::new()));
    let hianime = Arc::new(HiAnimeRust::new(Some(secret_config)).await);

    let router = Router::new()
        .route("/hianime/home", get(HiAnime::get_home))
        .route("/hianime/anime/:anime_id", get(HiAnime::get_anime_info))
        .route(
            "/hianime/anime/:anime_id/episodes",
            get(HiAnime::get_anime_episodes),
        )
        .route("/hianime/:category", get(HiAnime::get_category_results))
        .route("/hianime/search/:query", get(HiAnime::get_search_results))
        .route("/hianime/atoz-list", get(HiAnime::get_atoz_list_results))
        .layer(Extension(hianime))
        .layer(Extension(hianime_cache));

    Ok(router.into())
}
