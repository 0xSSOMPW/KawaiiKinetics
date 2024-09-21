use crate::error::KawaiiError;
use aniscraper::hianime::{
    AboutAnime, AtoZ, CategoryInfo, EpisodesInfo, HiAnimeRust, HomeInfo, SearchInfo,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use moka::future::Cache;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;

pub struct HiAnime {
    home_cache: Cache<String, HomeInfo>,
    anime_cache: Cache<String, AboutAnime>,
    episodes_cache: Cache<String, EpisodesInfo>,
    category_cache: Cache<(String, u32), CategoryInfo>,
    search_cache: Cache<(String, u32), SearchInfo>,
    atoz_cache: Cache<u32, AtoZ>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub page: Option<u32>,
}

impl HiAnime {
    pub fn new() -> Self {
        Self {
            home_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            anime_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            episodes_cache: Cache::builder()
                .time_to_live(Duration::from_secs(7200)) // 2 hours
                .build(),
            category_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
            search_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24 * 31)) // 1 month
                .build(),
            atoz_cache: Cache::builder()
                .time_to_live(Duration::from_secs(3600 * 24)) // 1 day
                .build(),
        }
    }

    pub async fn get_home(
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<HomeInfo>, KawaiiError> {
        let cache = cache.lock().await;
        let home_info = cache
            .home_cache
            .try_get_with_by_ref("home", async {
                hianime.scrape_home().await.map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(home_info))
    }

    pub async fn get_anime_info(
        Path(anime_id): Path<String>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<AboutAnime>, KawaiiError> {
        let cache = cache.lock().await;
        let anime_info = cache
            .anime_cache
            .try_get_with_by_ref(&anime_id, async {
                hianime
                    .scrape_about_anime(&anime_id)
                    .await
                    .map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(anime_info))
    }

    pub async fn get_anime_episodes(
        Path(anime_id): Path<String>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<EpisodesInfo>, KawaiiError> {
        let cache = cache.lock().await;
        let episodes_info = cache
            .episodes_cache
            .try_get_with_by_ref(&anime_id, async {
                hianime
                    .scrape_episodes(&anime_id)
                    .await
                    .map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(episodes_info))
    }

    pub async fn get_category_results(
        Path(category): Path<String>,
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<CategoryInfo>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let cache = cache.lock().await;
        let category_result = cache
            .category_cache
            .try_get_with_by_ref(&(category.clone(), page_no), async {
                hianime
                    .scrape_category(&category, page_no)
                    .await
                    .map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(category_result))
    }

    pub async fn get_search_results(
        Path(query): Path<String>,
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<SearchInfo>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let cache = cache.lock().await;
        let search_result = cache
            .search_cache
            .try_get_with_by_ref(&(query.clone(), page_no), async {
                hianime
                    .scrape_search(&query, page_no)
                    .await
                    .map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(search_result))
    }

    pub async fn get_atoz_list_results(
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
        Extension(cache): Extension<Arc<Mutex<HiAnime>>>,
    ) -> Result<Json<AtoZ>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let cache = cache.lock().await;
        let atoz_result = cache
            .atoz_cache
            .try_get_with_by_ref(&page_no, async {
                hianime
                    .scrape_atoz(page_no)
                    .await
                    .map_err(KawaiiError::from)
            })
            .await?;
        Ok(Json(atoz_result))
    }
}
