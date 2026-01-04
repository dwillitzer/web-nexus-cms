// Web Nexus Contracts - Layer 0: Foundation Contracts
//
// Shared type definitions for all crates.
// Auto-generates TypeScript bindings.
//
// Based on WEB-NEXUS V2 Architecture

#![allow(clippy::enum_variant_names)]

use serde::{Deserialize, Serialize};
use garde::Validate;
use utoipa::ToSchema;

// ============================================================================
// USER & AUTHENTICATION CONTRACTS
// ============================================================================

/// User account with role-based access control

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// Unique user ID
    pub id: String,
    /// Email address (must be valid)
    pub email: String,
    /// Display name
    pub name: String,
    /// Assigned roles
    pub roles: Vec<Role>,
    /// Account status
    pub status: UserStatus,
    /// Creation timestamp
    pub created_at: i64,
    /// Last login timestamp
    pub last_login: Option<i64>,
}

/// User role with granular permissions

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum Role {
    /// Full system access
    Admin,
    /// Content editing (shows, songs, photos, videos)
    Content,
    /// Media uploads only
    Media,
    /// View-only access to analytics
    ReadOnly,
    /// Site-specific access (e.g., band member)
    SiteEditor { site_id: String },
}

/// User account status

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserStatus {
    /// Account is active
    Active,
    /// Account is pending email verification
    Pending,
    /// Account is suspended
    Suspended,
    /// Account is deleted (soft delete)
    Deleted,
}

// ============================================================================
// CONTENT CONTRACTS - Shows
// ============================================================================

/// Live show or performance event

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Show {
    /// Unique show ID
    pub id: String,
    /// Site this show belongs to
    pub site_id: String,
    /// Event title
    pub title: String,
    /// Venue name
    pub venue: String,
    /// Venue address
    pub address: Option<String>,
    /// Event date (Unix timestamp)
    pub date: i64,
    /// Start time (HH:MM format)
    pub start_time: String,
    /// Ticket URL
    pub ticket_url: Option<String>,
    /// Event description
    pub description: Option<String>,
    /// Show status
    pub status: ShowStatus,
    /// Created by user ID
    pub created_by: String,
    /// Created timestamp
    pub created_at: i64,
    /// Last updated timestamp
    pub updated_at: i64,
}

/// Show status

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum ShowStatus {
    /// Upcoming show
    Upcoming,
    /// Show is currently happening
    Live,
    /// Show has ended
    Completed,
    /// Show was cancelled
    Cancelled,
}

// ============================================================================
// CONTENT CONTRACTS - Songs
// ============================================================================

/// Song in the band's repertoire

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Song {
    /// Unique song ID
    pub id: String,
    /// Site this song belongs to
    pub site_id: String,
    /// Song title
    pub title: String,
    /// Original artist (for covers)
    pub artist: Option<String>,
    /// Genre tags
    pub genres: Vec<String>,
    /// Duration in seconds
    pub duration_seconds: Option<i32>,
    /// Whether it's original or cover
    pub is_original: bool,
    /// Song key (musical key)
    pub musical_key: Option<String>,
    /// Notes for band members
    pub notes: Option<String>,
    /// Created timestamp
    pub created_at: i64,
}

/// Song list / setlist for a performance

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Setlist {
    /// Unique setlist ID
    pub id: String,
    /// Associated show ID
    pub show_id: Option<String>,
    /// List of song IDs in order
    pub song_ids: Vec<String>,
    /// Set name (e.g., "First Set", "Encore")
    pub name: Option<String>,
    /// Notes
    pub notes: Option<String>,
}

// ============================================================================
// CONTENT CONTRACTS - Photos & Media
// ============================================================================

/// Photo or image asset

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Photo {
    /// Unique photo ID
    pub id: String,
    /// Site this photo belongs to
    pub site_id: String,
    /// Original filename
    pub filename: String,
    /// CDN URL (full size)
    pub url_full: String,
    /// CDN URL (thumbnail)
    pub url_thumb: String,
    /// File size in bytes
    pub size_bytes: i64,
    /// Image dimensions
    pub dimensions: ImageDimensions,
    /// Alt text for accessibility
    pub alt_text: Option<String>,
    /// Photo caption
    pub caption: Option<String>,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Upload timestamp
    pub uploaded_at: i64,
    /// Uploaded by user ID
    pub uploaded_by: String,
}

/// Image dimensions

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ImageDimensions {
    pub width: i32,
    pub height: i32,
}

/// Photo gallery or album

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Gallery {
    /// Unique gallery ID
    pub id: String,
    /// Site this gallery belongs to
    pub site_id: String,
    /// Gallery title
    pub title: String,
    /// Gallery description
    pub description: Option<String>,
    /// Photo IDs in order
    pub photo_ids: Vec<String>,
    /// Cover photo ID
    pub cover_photo_id: Option<String>,
    /// Gallery visibility
    pub visibility: GalleryVisibility,
    /// Created timestamp
    pub created_at: i64,
}

/// Gallery visibility settings

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum GalleryVisibility {
    /// Publicly visible
    Public,
    /// Password protected
    Password { password: String },
    /// Only site members can see
    MembersOnly,
    /// Hidden (draft)
    Hidden,
}

// ============================================================================
// CONTENT CONTRACTS - Videos
// ============================================================================

/// Video asset (YouTube, Vimeo, or uploaded)

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Video {
    /// Unique video ID
    pub id: String,
    /// Site this video belongs to
    pub site_id: String,
    /// Video title
    pub title: String,
    /// Video description
    pub description: Option<String>,
    /// Video source
    pub source: VideoSource,
    /// Thumbnail URL
    pub thumbnail_url: Option<String>,
    /// Duration in seconds
    pub duration_seconds: Option<i32>,
    /// Video visibility
    pub visibility: GalleryVisibility,
    /// View count
    pub view_count: i64,
    /// Upload/publish timestamp
    pub published_at: i64,
}

/// Video hosting source
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum VideoSource {
    /// YouTube video
    YouTube { video_id: String },
    /// Vimeo video
    Vimeo { video_id: String },
    /// Direct upload (MP4, WebM)
    Direct { url: String },
    /// External embed
    External { embed_code: String },
}

// ============================================================================
// CONTENT CONTRACTS - Blog Posts
// ============================================================================

/// Blog post or news article

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BlogPost {
    /// Unique post ID
    pub id: String,
    /// Site this post belongs to
    pub site_id: String,
    /// Post title
    pub title: String,
    /// Post slug (URL-friendly)
    pub slug: String,
    /// Post content (Markdown or HTML)
    pub content: String,
    /// Post excerpt
    pub excerpt: Option<String>,
    /// Cover image ID
    pub cover_image_id: Option<String>,
    /// Author user ID
    pub author_id: String,
    /// Post status
    pub status: PostStatus,
    /// Published timestamp
    pub published_at: Option<i64>,
    /// Created timestamp
    pub created_at: i64,
    /// Updated timestamp
    pub updated_at: i64,
}

/// Blog post status

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum PostStatus {
    /// Draft - not published
    Draft,
    /// Scheduled for future publishing
    Scheduled,
    /// Published
    Published,
    /// Archived (not shown in lists)
    Archived,
}

// ============================================================================
// SITE CONTRACTS
// ============================================================================

/// Website / deployment target

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct Site {
    /// Unique site ID
    pub id: String,
    /// Site identifier (used in URLs)
    pub slug: String,
    /// Site name
    pub name: String,
    /// Custom domain
    pub domain: Option<String>,
    /// Site description
    pub description: Option<String>,
    /// Site owner user ID
    pub owner_id: String,
    /// Site members (user IDs with access)
    pub member_ids: Vec<String>,
    /// Site theme
    pub theme: String,
    /// Site configuration (JSON)
    pub config: serde_json::Value,
    /// Site status
    pub status: SiteStatus,
    /// Created timestamp
    pub created_at: i64,
}

/// Site deployment status

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq, Eq, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum SiteStatus {
    /// Site is active and deployed
    Active,
    /// Site is being built
    Building,
    /// Site deployment failed
    Failed,
    /// Site is suspended
    Suspended,
    /// Site is archived (not deployed)
    Archived,
}

// ============================================================================
// BAND/MEMBER SPECIFIC CONTRACTS
// ============================================================================

/// Band member profile

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BandMember {
    /// Unique member ID
    pub id: String,
    /// Site this member belongs to
    pub site_id: String,
    /// Member name
    pub name: String,
    /// Instrument or role
    pub role: String,
    /// Member bio
    pub bio: Option<String>,
    /// Profile photo ID
    pub photo_id: Option<String>,
    /// Contact email
    pub email: Option<String>,
    /// Display order on site
    pub display_order: i32,
}

/// Contact form submission

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ContactSubmission {
    /// Unique submission ID
    #[garde(skip)]
    pub id: String,
    /// Site this submission is for
    #[garde(skip)]
    pub site_id: String,
    /// Submitter name
    #[garde(skip)]
    pub name: String,
    /// Submitter email
    #[garde(email)]
    pub email: String,
    /// Message subject
    #[garde(skip)]
    pub subject: Option<String>,
    /// Message content
    #[garde(skip)]
    pub message: String,
    /// Submission timestamp
    #[garde(skip)]
    pub submitted_at: i64,
    /// Whether submission was read
    #[garde(skip)]
    pub is_read: bool,
}

// ============================================================================
// API REQUEST/RESPONSE CONTRACTS
// ============================================================================

/// Paginated response

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedResponse<T> {
    /// Data items
    pub data: Vec<T>,
    /// Current page number (1-indexed)
    pub page: i32,
    /// Items per page
    pub per_page: i32,
    /// Total items across all pages
    pub total: i64,
    /// Total pages
    pub total_pages: i32,
    /// Has next page?
    pub has_next: bool,
    /// Has previous page?
    pub has_prev: bool,
}

/// API error response

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ApiError {
    /// Error code
    pub code: String,
    /// Error message
    pub message: String,
    /// Additional details
    pub details: Option<serde_json::Value>,
    /// Request ID for tracing
    pub request_id: Option<String>,
}

/// Authentication request

#[derive(Serialize, Deserialize, Debug, Clone, Validate, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequest {
    /// Email address
    #[garde(email)]
    pub email: String,
    /// Password
    #[garde(length(min = 8))]
    pub password: String,
}

/// Authentication response

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponse {
    /// Authentication token
    pub token: String,
    /// User info
    pub user: User,
    /// Token expires at
    pub expires_at: i64,
}

// ============================================================================
// HELPER FUNCTIONS FOR WASM
// ============================================================================


impl User {
    /// Check if user has admin role
    pub fn is_admin(&self) -> bool {
        self.roles.contains(&Role::Admin)
    }

    /// Check if user can edit content
    pub fn can_edit_content(&self) -> bool {
        self.roles.contains(&Role::Admin)
            || self.roles.contains(&Role::Content)
            || self.roles.iter().any(|r| matches!(r, Role::SiteEditor { .. }))
    }

    /// Check if user can upload media
    pub fn can_upload_media(&self) -> bool {
        self.roles.contains(&Role::Admin)
            || self.roles.contains(&Role::Content)
            || self.roles.contains(&Role::Media)
    }
}


impl Site {
    /// Get the default domain for this site
    pub fn default_domain(&self) -> String {
        format!("{}.webnexus.dev", self.slug)
    }
}

// ============================================================================
// VALIDATION HELPERS
// ============================================================================

/// Custom validation: date must be in the future
fn is_future_date(date: i64) -> Result<(), garde::Error> {
    let now = chrono::Utc::now().timestamp();
    if date > now {
        Ok(())
    } else {
        Err(garde::Error::new("date must be in the future"))
    }
}

// ============================================================================
// TYPE GENERATION MACROS
// ============================================================================

/// This module is used by the build process to generate TypeScript types
#[cfg(feature = "typescript")]
pub mod typescript {
    use super::*;

    /// Export all types for TypeScript generation
    pub fn export_types() -> String {
        // This would be used by a build script to generate .d.ts files
        format!(
            r#"
// Auto-generated TypeScript types from Rust contracts

export interface User {{
  id: string;
  email: string;
  name: string;
  roles: Role[];
  status: UserStatus;
  createdAt: number;
  lastLogin?: number;
}}

export type Role =
  | {{ type: "Admin" }}
  | {{ type: "Content" }}
  | {{ type: "Media" }}
  | {{ type: "ReadOnly" }}
  | {{ type: "SiteEditor", siteId: string }};

export type UserStatus = "Active" | "Pending" | "Suspended" | "Deleted";

// ... (more types would be generated)
"#
        )
    }
}
