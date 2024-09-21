use std::sync::Arc;

use aniscraper::hianime::{
    AboutAnime, AtoZ, CategoryInfo, EpisodesInfo, HiAnimeRust, HomeInfo, SearchInfo,
};
use axum::{
    extract::{Path, Query},
    Extension, Json,
};
use serde::{Deserialize, Serialize};

use crate::error::KawaiiError;

#[derive(Debug, Serialize, Deserialize)]
pub struct SearchQuery {
    pub page: Option<u32>,
}

pub struct HiAnime {}

impl HiAnime {
    pub async fn get_home(
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<HomeInfo>, KawaiiError> {
        let home_info = hianime.scrape_home().await?;
        Ok(Json(home_info))
    }

    pub async fn get_anime_info(
        Path(anime_id): Path<String>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<AboutAnime>, KawaiiError> {
        let anime_info = hianime.scrape_about_anime(&anime_id).await?;
        Ok(Json(anime_info))
    }

    pub async fn get_anime_episodes(
        Path(anime_id): Path<String>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<EpisodesInfo>, KawaiiError> {
        let episodes_info = hianime.scrape_episodes(&anime_id).await?;
        Ok(Json(episodes_info))
    }

    pub async fn get_category_results(
        Path(category): Path<String>,
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<CategoryInfo>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let category_result = hianime.scrape_category(&category, page_no).await?;
        Ok(Json(category_result))
    }

    pub async fn get_search_results(
        Path(query): Path<String>,
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<SearchInfo>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let search_result = hianime.scrape_search(&query, page_no).await?;
        Ok(Json(search_result))
    }

    pub async fn get_atoz_list_results(
        Query(query_params): Query<SearchQuery>,
        Extension(hianime): Extension<Arc<HiAnimeRust>>,
    ) -> Result<Json<AtoZ>, KawaiiError> {
        let page_no = query_params.page.unwrap_or(1);
        let atoz_result = hianime.scrape_atoz(page_no).await?;
        Ok(Json(atoz_result))
    }
}
