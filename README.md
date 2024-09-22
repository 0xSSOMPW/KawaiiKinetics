# KawaiiKinetics

KawaiiKinetics is a Rust API built with Axum and Shuttle, utilizing the Aniscraper library for scraping anime-related content. This API provides various endpoints to retrieve information about anime, including trending series, episode details, and more.

## Features

- **Anime Information Retrieval**: Get details on various anime, including home info, specific anime info, episode listings, and more.
- **Caching**: Utilizes caching for better performance.
- **Secret Management**: Securely handles configuration and secrets using a dedicated secret store.

## Getting Started

### Prerequisites

- Rust (1.50 or later)
- Cargo
- An instance of [Shuttle](https://shuttle.rs) for deployment (if needed)

### Installation

1. Clone the repository:

   ```bash
   git clone https://github.com/0xSSOMPW/KawaiiKinetics.git
   cd KawaiiKinetics
   ```
   
2. Run the application:

   ```bash
   cargo run
   ```

### Configuration

Set up your secret configuration in your environment. The following keys are required:

- `MAX_RETRIES_ATTEMPTS`
- `REQWEST_ERROR_WEBHOOK`
- `NO_PROXIES_AVAILABLE_ERROR_WEBHOOK`
- `FAILED_TO_FETCH_AFTER_RETRIES_ERROR_WEBHOOK`
- `UTILS_ERROR_WEBHOOK`
- `UNKNOWN_ERROR_WEBHOOK`
- `HTTP_URL`
- `SOCK4_URL`
- `SOCK5_URL`
- `HIANIME_DOMAINS`
- `USER_AGENT_HEADER`          // REQUIRED
- `ACCEPT_ENCODING_HEADER`     // REQUIRED
- `ACCEPT_HEADER`       // REQUIRED

These secrets will be loaded into the application at runtime.

## API Endpoints

### 1. Home Information

- **Endpoint**: `GET /hianime/home`
- **Returns**: `HomeInfo`
  
```json
{
    "trending": [/* Array of MinimalAnime */],
    "latest_episodes": [/* Array of Anime */],
    "top_upcoming_animes": [/* Array of Anime */],
    "spotlight_animes": [/* Array of SpotlightAnime */],
    "featured": {/* FeaturedAnime */},
    "top_10_animes": {/* Top10PeriodRankedAnime */},
    "genres": [/* Array of Strings */]
}
```

### 2. Anime Information

- **Endpoint**: `GET /hianime/anime/:anime_id`
- **Returns**: `AboutAnime`
  
```json
{
    "id": "string",
    "mal_id": 123,
    "al_id": 456,
    "anime_id": 789,
    "title": "string",
    "description": "string",
    "image": "string",
    "rating": "string",
    "category": "string",
    "duration": "string",
    "quality": "string",
    "subs": 12,
    "dubs": 34,
    "eps": 24,
    "japanese": "string",
    "synonyms": "string",
    "aired": "string",
    "premiered": "string",
    "status": "string",
    "mal_score": "string",
    "studios": ["string"],
    "producers": ["string"],
    "genres": ["string"],
    "most_popular_animes": [/* Array of SideBarAnimes */],
    "related_animes": [/* Array of SideBarAnimes */],
    "recommended_animes": [/* Array of Anime */],
    "seasons": [/* Array of AnimeSeason */]
}
```

### 3. Category Results

- **Endpoint**: `GET /hianime/:category`
- **Returns**: `CategoryInfo`
  
```json
{
    "total_pages": 5,
    "current_page": 1,
    "has_next_page": true,
    "animes": [/* Array of Anime */],
    "top_10_animes": {/* Top10PeriodRankedAnime */},
    "genres": [/* Array of Strings */]
}
```

### 4. Anime Episodes

- **Endpoint**: `GET /hianime/anime/:anime_id/episodes`
- **Returns**: `EpisodesInfo`
  
```json
{
    "total_episodes": 24,
    "episodes": [/* Array of AnimeEpisode */]
}
```

### 5. Search Results

- **Endpoint**: `GET /hianime/search/:query`
- **Returns**: `SearchInfo`
  
```json
{
    "total_pages": 3,
    "current_page": 1,
    "has_next_page": true,
    "animes": [/* Array of Anime */],
    "most_popular_animes": [/* Array of SideBarAnimes */],
    "genres": [/* Array of Strings */]
}
```

### 6. A-Z List

- **Endpoint**: `GET /hianime/atoz-list`
- **Returns**: `AtoZ`
  
```json
{
    "has_next_page": true,
    "current_page": 1,
    "total_pages": 10,
    "animes": [/* Array of Anime */]
}
```

### 7. Episode Source

- **Endpoint**: `GET /hianime/episode-src/:anime_id`
- **Returns**: `ServerInfo`
  
```json
{
    "episode_no": 1,
    "sub": [/* Array of Server */],
    "dub": [/* Array of Server */]
}
```

### 8. Streaming Links

- **Endpoint**: `GET /hianime/episode-src-links/:anime_id`
- **Returns**: `ServerExtractedInfo`
  
```json
{
    "MegaCloud": {
        "intro": {/* IntroOutro */},
        "outro": {/* IntroOutro */},
        "tracks": [/* Array of Track */],
        "sources": [/* Array of Source */]
    }
}
```

### Error Handling

All API responses are returned as JSON. In the case of an error, a response will be structured as follows:

```json
{
    "error": "Error message here"
}
```

## Contributing

Contributions are welcome! If you'd like to contribute to KawaiiKinetics, please fork the repository and submit a pull request.

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

## Acknowledgments

- [Axum](https://github.com/tokio-rs/axum) for the web framework.
- [Shuttle](https://shuttle.rs) for seamless deployment.
- [Aniscraper](https://github.com/0xSSOMPW/aniscraper) for anime scraping functionality.
