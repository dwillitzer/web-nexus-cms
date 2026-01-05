// Web Nexus API - Cloudflare Workers API Layer
//
// RESTful API with authentication, CRUD operations, and WebSocket support
//
// Based on WEB-NEXUS V2 Architecture - Section 3.1

use worker::*;
use serde_json::json;
use web_nexus_contracts::{
    Show, Song, Photo, Video, BlogPost, CreateShowRequest, UpdateShowRequest,
    CreateSongRequest, CreateBlogPostRequest, CreatePhotoRequest, CreateVideoRequest,
    ApiErrorKind, PaginatedResponse, ShowStatus, PostStatus, GalleryVisibility, VideoSource,
    ImageDimensions,
};
use web_nexus_state::AppState;
use std::sync::Arc;
use tokio::sync::RwLock;
use garde::Validate;

/// Shared application state for the Workers
#[derive(Clone)]
pub struct ApiState {
    pub app_state: Arc<RwLock<AppState>>,
    pub jwt_secret: String,
}

impl ApiState {
    /// Create a new API state
    pub fn new() -> Self {
        Self {
            app_state: Arc::new(RwLock::new(AppState::new())),
            jwt_secret: std::env::var("JWT_SECRET").unwrap_or_else(|_| "dev-secret".to_string()),
        }
    }

    /// Get a show by ID
    pub async fn get_show(&self, id: &str) -> Option<Show> {
        let state = self.app_state.read().await;
        state.shows.get(id).cloned()
    }

    /// List all shows with optional pagination
    pub async fn list_shows(&self, page: u32, per_page: u32) -> Vec<Show> {
        let state = self.app_state.read().await;
        let shows: Vec<Show> = state.shows.values().cloned().collect();
        let start = (page * per_page) as usize;
        let end = ((page + 1) * per_page) as usize;
        shows.into_iter().skip(start).take(end - start).collect()
    }

    /// Create a new show
    pub async fn create_show(&self, show: Show) -> std::result::Result<Show, ApiErrorKind> {
        let mut state = self.app_state.write().await;
        let id = show.id.clone();
        state.shows.insert(id.clone(), show.clone());
        Ok(show)
    }

    /// Update an existing show
    pub async fn update_show(&self, id: &str, show: Show) -> std::result::Result<Show, ApiErrorKind> {
        let mut state = self.app_state.write().await;
        if !state.shows.contains_key(id) {
            return Err(ApiErrorKind::NotFound("Show not found".to_string()));
        }
        state.shows.insert(id.to_string(), show.clone());
        Ok(show)
    }

    /// Delete a show
    pub async fn delete_show(&self, id: &str) -> std::result::Result<(), ApiErrorKind> {
        let mut state = self.app_state.write().await;
        state.shows.remove(id)
            .map(|_| ())
            .ok_or_else(|| ApiErrorKind::NotFound("Show not found".to_string()))
    }

    /// Get all songs
    pub async fn list_songs(&self) -> Vec<Song> {
        let state = self.app_state.read().await;
        state.songs.values().cloned().collect()
    }

    /// Create a song
    pub async fn create_song(&self, song: Song) -> std::result::Result<Song, ApiErrorKind> {
        let mut state = self.app_state.write().await;
        let id = song.id.clone();
        state.songs.insert(id.clone(), song.clone());
        Ok(song)
    }

    /// Get blog posts with pagination
    pub async fn list_posts(&self, page: u32, per_page: u32) -> Vec<BlogPost> {
        let state = self.app_state.read().await;
        let posts: Vec<BlogPost> = state.posts.values().cloned().collect();
        let start = (page * per_page) as usize;
        let end = ((page + 1) * per_page) as usize;
        posts.into_iter().skip(start).take(end - start).collect()
    }

    /// Create a blog post
    pub async fn create_post(&self, post: BlogPost) -> std::result::Result<BlogPost, ApiErrorKind> {
        let mut state = self.app_state.write().await;
        let id = post.id.clone();
        state.posts.insert(id.clone(), post.clone());
        Ok(post)
    }

    /// Update a blog post
    pub async fn update_post(&self, id: &str, post: BlogPost) -> std::result::Result<BlogPost, ApiErrorKind> {
        let mut state = self.app_state.write().await;
        if !state.posts.contains_key(id) {
            return Err(ApiErrorKind::NotFound("Blog post not found".to_string()));
        }
        state.posts.insert(id.to_string(), post.clone());
        Ok(post)
    }

    /// Delete a blog post
    pub async fn delete_post(&self, id: &str) -> std::result::Result<(), ApiErrorKind> {
        let mut state = self.app_state.write().await;
        state.posts.remove(id)
            .map(|_| ())
            .ok_or_else(|| ApiErrorKind::NotFound("Blog post not found".to_string()))
    }

    /// Get photos with pagination
    pub async fn list_photos(&self, page: u32, per_page: u32) -> Vec<Photo> {
        let state = self.app_state.read().await;
        let photos: Vec<Photo> = state.photos.values().cloned().collect();
        let start = (page * per_page) as usize;
        let end = ((page + 1) * per_page) as usize;
        photos.into_iter().skip(start).take(end - start).collect()
    }

    /// Get all videos
    pub async fn list_videos(&self) -> Vec<Video> {
        let state = self.app_state.read().await;
        state.videos.values().cloned().collect()
    }
}

/// Extract user ID from JWT token (stub implementation)
fn extract_user_id(_req: &Request) -> worker::Result<String> {
    // TODO: Implement proper JWT validation
    Ok("user-123".to_string()) // Stub for now
}

/// Check if user has required permission (stub implementation)
fn check_permission(_user_id: &str, _permission: &str) -> worker::Result<()> {
    // TODO: Implement RBAC check
    Ok(()) // Stub for now
}

/// Helper: Convert ApiErrorKind to Worker Response
fn error_response(error: ApiErrorKind) -> worker::Result<Response> {
    let status = match &error {
        ApiErrorKind::NotFound(_) => 404,
        ApiErrorKind::Unauthorized => 401,
        ApiErrorKind::Forbidden => 403,
        ApiErrorKind::ValidationError(_) => 400,
        ApiErrorKind::Internal(_) => 500,
    };

    Response::error(format!("{}", error), status)
}

/// Helper: Parse ID from path
fn extract_id(req: &Request) -> worker::Result<String> {
    let url = req.url()?;
    let path_segments: Vec<&str> = url.path_segments().map(|s| s.collect()).unwrap_or_default();
    path_segments
        .last()
        .map(|s| s.to_string())
        .ok_or_else(|| worker::Error::from("Missing ID in path"))
}

/// Helper: Parse query parameter with default
fn parse_query_param<T: std::str::FromStr>(
    req: &Request,
    param: &str,
    default: T,
) -> T {
    let url = match req.url() {
        Ok(u) => u,
        Err(_) => return default,
    };

    url.query_pairs()
        .find(|(k, _)| k == param)
        .and_then(|(_, v)| v.parse().ok())
        .unwrap_or(default)
}

// ============================================================================
// Root Handlers
// ============================================================================

/// Root API handler
pub fn root_handler() -> worker::Result<Response> {
    Response::from_json(&json!({
        "version": env!("CARGO_PKG_VERSION"),
        "status": "ok",
        "message": "Web Nexus CMS API",
        "endpoints": {
            "health": "/health",
            "shows": "/api/shows",
            "songs": "/api/songs",
            "posts": "/api/posts",
            "photos": "/api/photos",
            "videos": "/api/videos"
        }
    }))
}

/// Health check endpoint
pub fn health_handler() -> worker::Result<Response> {
    Response::from_json(&json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339()
    }))
}

// ============================================================================
// Shows Handlers
// ============================================================================

pub mod shows {
    use super::*;

    /// GET /api/shows - List all shows with pagination
    pub async fn list(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let page: u32 = parse_query_param(&req, "page", 0u32);
        let per_page: u32 = parse_query_param(&req, "per_page", 20u32);

        let shows = ctx.data.list_shows(page, per_page).await;
        let total = ctx.data.app_state.read().await.shows.len() as i64;
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

        let response = PaginatedResponse {
            data: shows,
            page: page as i32,
            per_page: per_page as i32,
            total,
            total_pages,
            has_next: (page as i32 + 1) < total_pages,
            has_prev: page > 0,
        };

        Response::from_json(&response)
    }

    /// GET /api/shows/:id - Get a specific show
    pub async fn get(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let id = extract_id(&req)?;

        match ctx.data.get_show(&id).await {
            Some(show) => Response::from_json(&show),
            None => error_response(ApiErrorKind::NotFound("Show not found".to_string())),
        }
    }

    /// POST /api/shows - Create a new show
    pub async fn create(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let _user_id = extract_user_id(&req)?;
        check_permission(&_user_id, "create_shows")?;

        let body = req.json().await?;
        let create_req: CreateShowRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        // Validate using garde
        if let Err(errors) = create_req.validate() {
            return error_response(ApiErrorKind::ValidationError(format!("Validation failed: {:?}", errors)));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let show = Show {
            id: id.clone(),
            site_id: create_req.site_id,
            title: format!("Show at {}", create_req.venue), // Default title
            venue: create_req.venue,
            address: create_req.city,
            date: create_req.date,
            start_time: "20:00".to_string(), // Default start time
            ticket_url: None,
            description: None,
            status: ShowStatus::Upcoming,
            created_by: _user_id,
            created_at: now,
            updated_at: now,
        };

        match ctx.data.create_show(show.clone()).await {
            Ok(_) => Response::from_json(&show),
            Err(e) => error_response(e),
        }
    }

    /// PUT /api/shows/:id - Update a show
    pub async fn update(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "update_shows")?;

        let id = extract_id(&req)?;
        let body = req.json().await?;
        let update_req: UpdateShowRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        // Get existing show
        let mut existing = match ctx.data.get_show(&id).await {
            Some(show) => show,
            None => return error_response(ApiErrorKind::NotFound("Show not found".to_string())),
        };

        // Update fields
        if let Some(date) = update_req.date {
            existing.date = date;
        }
        if let Some(venue) = update_req.venue {
            existing.venue = venue;
        }
        if let Some(city) = update_req.city {
            existing.address = Some(city);
        }
        if let Some(setlist_id) = update_req.setlist_id {
            // Store setlist_id in description for now (contracts need update)
            existing.description = Some(setlist_id);
        }
        existing.updated_at = chrono::Utc::now().timestamp();

        match ctx.data.update_show(&id, existing.clone()).await {
            Ok(_) => Response::from_json(&existing),
            Err(e) => error_response(e),
        }
    }

    /// DELETE /api/shows/:id - Delete a show
    pub async fn delete(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "delete_shows")?;

        let id = extract_id(&req)?;

        match ctx.data.delete_show(&id).await {
            Ok(_) => Response::empty().map(|r| r.with_status(204)),
            Err(e) => error_response(e),
        }
    }
}

// ============================================================================
// Songs Handlers
// ============================================================================

pub mod songs {
    use super::*;

    /// GET /api/songs - List all songs
    pub async fn list(_req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let songs = ctx.data.list_songs().await;
        Response::from_json(&songs)
    }

    /// POST /api/songs - Create a new song
    pub async fn create(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "create_songs")?;

        let body = req.json().await?;
        let create_req: CreateSongRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        if let Err(errors) = create_req.validate() {
            return error_response(ApiErrorKind::ValidationError(format!("Validation failed: {:?}", errors)));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let song = Song {
            id: id.clone(),
            site_id: "default".to_string(), // TODO: Get from request
            title: create_req.title,
            artist: create_req.artist,
            genres: vec![],
            duration_seconds: create_req.duration_seconds.map(|d| d as i32),
            is_original: true,
            musical_key: None,
            notes: None,
            created_at: now,
        };

        match ctx.data.create_song(song.clone()).await {
            Ok(_) => Response::from_json(&song),
            Err(e) => error_response(e),
        }
    }
}

// ============================================================================
// Blog Posts Handlers
// ============================================================================

pub mod posts {
    use super::*;

    /// GET /api/posts - List blog posts with pagination
    pub async fn list(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let page: u32 = parse_query_param(&req, "page", 0u32);
        let per_page: u32 = parse_query_param(&req, "per_page", 20u32);

        let posts = ctx.data.list_posts(page, per_page).await;
        let total = ctx.data.app_state.read().await.posts.len() as i64;
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

        let response = PaginatedResponse {
            data: posts,
            page: page as i32,
            per_page: per_page as i32,
            total,
            total_pages,
            has_next: (page as i32 + 1) < total_pages,
            has_prev: page > 0,
        };

        Response::from_json(&response)
    }

    /// POST /api/posts - Create a new blog post
    pub async fn create(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "create_posts")?;

        let body = req.json().await?;
        let create_req: CreateBlogPostRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        if let Err(errors) = create_req.validate() {
            return error_response(ApiErrorKind::ValidationError(format!("Validation failed: {:?}", errors)));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        let post = BlogPost {
            id: id.clone(),
            site_id: create_req.site_id,
            title: create_req.title,
            slug: create_req.slug,
            content: create_req.content,
            excerpt: create_req.excerpt,
            cover_image_id: create_req.featured_image,
            author_id: user_id,
            status: if create_req.published.unwrap_or(false) {
                PostStatus::Published
            } else {
                PostStatus::Draft
            },
            published_at: if create_req.published.unwrap_or(false) {
                Some(create_req.published_at.unwrap_or(now))
            } else {
                None
            },
            created_at: now,
            updated_at: now,
        };

        match ctx.data.create_post(post.clone()).await {
            Ok(_) => Response::from_json(&post),
            Err(e) => error_response(e),
        }
    }
}

// ============================================================================
// Photos Handlers
// ============================================================================

pub mod photos {
    use super::*;

    /// GET /api/photos - List photos with pagination
    pub async fn list(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let page: u32 = parse_query_param(&req, "page", 0u32);
        let per_page: u32 = parse_query_param(&req, "per_page", 20u32);

        let photos = ctx.data.list_photos(page, per_page).await;
        let total = ctx.data.app_state.read().await.photos.len() as i64;
        let total_pages = ((total as f64) / (per_page as f64)).ceil() as i32;

        let response = PaginatedResponse {
            data: photos,
            page: page as i32,
            per_page: per_page as i32,
            total,
            total_pages,
            has_next: (page as i32 + 1) < total_pages,
            has_prev: page > 0,
        };

        Response::from_json(&response)
    }

    /// POST /api/photos - Create a new photo
    pub async fn create(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "create_photos")?;

        let body = req.json().await?;
        let create_req: CreatePhotoRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        if let Err(errors) = create_req.validate() {
            return error_response(ApiErrorKind::ValidationError(format!("Validation failed: {:?}", errors)));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        // Extract filename from URL
        let filename = create_req.url.split('/').last().unwrap_or("photo.jpg").to_string();

        let photo = Photo {
            id: id.clone(),
            site_id: create_req.site_id,
            filename,
            url_full: create_req.url.clone(),
            url_thumb: create_req.thumbnail_url.unwrap_or_else(|| create_req.url.clone()),
            size_bytes: 0,
            dimensions: ImageDimensions { width: 0, height: 0 },
            alt_text: Some(create_req.title),
            caption: create_req.caption,
            tags: vec![],
            uploaded_at: now,
            uploaded_by: user_id,
        };

        // Add to state
        let mut state = ctx.data.app_state.write().await;
        state.photos.insert(id.clone(), photo.clone());

        Response::from_json(&photo)
    }
}

// ============================================================================
// Videos Handlers
// ============================================================================

pub mod videos {
    use super::*;

    /// GET /api/videos - List all videos
    pub async fn list(_req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let videos = ctx.data.list_videos().await;
        Response::from_json(&videos)
    }

    /// POST /api/videos - Create a new video
    pub async fn create(mut req: Request, ctx: RouteContext<ApiState>) -> worker::Result<Response> {
        let user_id = extract_user_id(&req)?;
        check_permission(&user_id, "create_videos")?;

        let body = req.json().await?;
        let create_req: CreateVideoRequest = serde_json::from_value(body)
            .map_err(|e| worker::Error::from(format!("Invalid request: {}", e)))?;

        if let Err(errors) = create_req.validate() {
            return error_response(ApiErrorKind::ValidationError(format!("Validation failed: {:?}", errors)));
        }

        let id = uuid::Uuid::new_v4().to_string();
        let now = chrono::Utc::now().timestamp();

        // Parse video URL to determine source
        let source = if let Some(video_type) = create_req.video_type {
            match video_type.as_str() {
                "youtube" => VideoSource::YouTube { video_id: extract_video_id(&create_req.url) },
                "vimeo" => VideoSource::Vimeo { video_id: extract_video_id(&create_req.url) },
                _ => VideoSource::Direct { url: create_req.url.clone() },
            }
        } else {
            VideoSource::Direct { url: create_req.url.clone() }
        };

        let video = Video {
            id: id.clone(),
            site_id: create_req.site_id,
            title: create_req.title,
            description: create_req.description,
            source,
            thumbnail_url: create_req.thumbnail_url,
            duration_seconds: create_req.duration_seconds.map(|d| d as i32),
            visibility: GalleryVisibility::Public,
            view_count: 0,
            published_at: now,
        };

        // Add to state
        let mut state = ctx.data.app_state.write().await;
        state.videos.insert(id.clone(), video.clone());

        Response::from_json(&video)
    }

    /// Helper: Extract video ID from URL
    fn extract_video_id(url: &str) -> String {
        // Simple extraction - in production would use proper URL parsing
        url.split('/').last().unwrap_or_default().to_string()
    }
}

// ============================================================================
// Middleware
// ============================================================================

pub mod middleware {
    use super::*;

    /// Authentication middleware - validates JWT token
    pub fn auth(req: &mut Request) -> worker::Result<Option<String>> {
        let auth_header_result = req.headers().get("Authorization");

        let auth_header = match auth_header_result {
            Ok(Some(header)) => header.to_string(),
            Ok(None) => return Ok(None),
            Err(_) => return Ok(None),
        };

        if !auth_header.starts_with("Bearer ") {
            return Ok(None);
        }

        let token = auth_header[7..].to_string();
        // TODO: Validate JWT token
        Ok(Some(token))
    }

    /// CORS middleware - adds CORS headers
    pub fn cors(res: &mut Response) -> worker::Result<()> {
        let headers = res.headers_mut();
        headers.set("Access-Control-Allow-Origin", "*")?;
        headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")?;
        headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization")?;
        Ok(())
    }
}
