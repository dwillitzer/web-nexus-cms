// Web Nexus State - Layer 2: CRDT State Management
//
// Local-first state with CRDT merge and edge synchronization
//
// Based on WEB-NEXUS V2 Architecture - Section 2.1

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;
use web_nexus_contracts::{Show, Song, Photo, Video, BlogPost, Site, User};

/// State synchronization error
#[derive(Error, Debug)]
pub enum SyncError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Serialization error: {0}")]
    Serialization(String),

    #[error("Merge conflict: {0}")]
    MergeConflict(String),

    #[error("Authentication required")]
    Unauthorized,
}

/// Sync status for state
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SyncStatus {
    /// Synced with edge
    Synced,
    /// Local changes pending sync
    Pending,
    /// Sync in progress
    Syncing,
    /// Sync failed
    Failed(String),
}

/// Clock type for CRDT operations
pub type Clock = u64;

/// Application state using CRDTs for conflict-free replication
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppState {
    /// All sites
    pub sites: HashMap<String, Site>,
    /// All shows
    pub shows: HashMap<String, Show>,
    /// All songs
    pub songs: HashMap<String, Song>,
    /// All photos
    pub photos: HashMap<String, Photo>,
    /// All videos
    pub videos: HashMap<String, Video>,
    /// All blog posts
    pub posts: HashMap<String, BlogPost>,
    /// All users
    pub users: HashMap<String, User>,
    /// Sync status
    pub sync_status: SyncStatus,
    /// Last sync timestamp
    pub last_sync: Option<i64>,
    /// Logical clock for this replica
    pub clock: Clock,
}

impl AppState {
    /// Create new empty state
    pub fn new() -> Self {
        Self {
            sites: HashMap::new(),
            shows: HashMap::new(),
            songs: HashMap::new(),
            photos: HashMap::new(),
            videos: HashMap::new(),
            posts: HashMap::new(),
            users: HashMap::new(),
            sync_status: SyncStatus::Synced,
            last_sync: None,
            clock: 0,
        }
    }

    /// Merge another state into this one (CRDT merge - last writer wins by updated_at)
    pub fn merge(&mut self, other: AppState) {
        // Merge sites (last writer wins by created_at - simpler since Site doesn't have updated_at)
        for (id, site) in other.sites {
            if let Some(existing) = self.sites.get(&id) {
                if site.created_at > existing.created_at {
                    self.sites.insert(id, site);
                }
            } else {
                self.sites.insert(id, site);
            }
        }

        // Merge shows
        for (id, show) in other.shows {
            if let Some(existing) = self.shows.get(&id) {
                if show.updated_at > existing.updated_at {
                    self.shows.insert(id, show);
                }
            } else {
                self.shows.insert(id, show);
            }
        }

        // Merge songs
        for (id, song) in other.songs {
            self.songs.insert(id, song);
        }

        // Merge photos
        for (id, photo) in other.photos {
            self.photos.insert(id, photo);
        }

        // Merge videos
        for (id, video) in other.videos {
            self.videos.insert(id, video);
        }

        // Merge posts
        for (id, post) in other.posts {
            if let Some(existing) = self.posts.get(&id) {
                if post.updated_at > existing.updated_at {
                    self.posts.insert(id, post);
                }
            } else {
                self.posts.insert(id, post);
            }
        }

        // Merge users
        for (id, user) in other.users {
            self.users.insert(id, user);
        }

        // Update clock (take max)
        self.clock = self.clock.max(other.clock);

        // Update sync status
        self.sync_status = SyncStatus::Synced;
        self.last_sync = Some(chrono::Utc::now().timestamp());
    }

    /// Get all shows for a site
    pub fn get_site_shows(&self, site_id: &str) -> Vec<Show> {
        self.shows
            .values()
            .filter(|s| s.site_id == site_id)
            .cloned()
            .collect()
    }

    /// Get all songs for a site
    pub fn get_site_songs(&self, site_id: &str) -> Vec<Song> {
        self.songs
            .values()
            .filter(|s| s.site_id == site_id)
            .cloned()
            .collect()
    }

    /// Get all photos for a site
    pub fn get_site_photos(&self, site_id: &str) -> Vec<Photo> {
        self.photos
            .values()
            .filter(|p| p.site_id == site_id)
            .cloned()
            .collect()
    }

    /// Add a new show
    pub fn add_show(&mut self, show: Show) -> Result<(), SyncError> {
        self.clock += 1;
        self.shows.insert(show.id.clone(), show);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Update an existing show
    pub fn update_show(&mut self, show: Show) -> Result<(), SyncError> {
        self.clock += 1;
        self.shows.insert(show.id.clone(), show);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Delete a show
    pub fn delete_show(&mut self, show_id: &str) -> Result<(), SyncError> {
        self.clock += 1;
        self.shows.remove(show_id);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Add a song
    pub fn add_song(&mut self, song: Song) -> Result<(), SyncError> {
        self.clock += 1;
        self.songs.insert(song.id.clone(), song);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Update a song
    pub fn update_song(&mut self, song: Song) -> Result<(), SyncError> {
        self.clock += 1;
        self.songs.insert(song.id.clone(), song);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Add a photo
    pub fn add_photo(&mut self, photo: Photo) -> Result<(), SyncError> {
        self.clock += 1;
        self.photos.insert(photo.id.clone(), photo);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Add a blog post
    pub fn add_post(&mut self, post: BlogPost) -> Result<(), SyncError> {
        self.clock += 1;
        self.posts.insert(post.id.clone(), post);
        self.sync_status = SyncStatus::Pending;
        Ok(())
    }

    /// Check if sync is needed
    pub fn needs_sync(&self) -> bool {
        matches!(self.sync_status, SyncStatus::Pending)
    }

    /// Mark sync as started
    pub fn mark_syncing(&mut self) {
        self.sync_status = SyncStatus::Syncing;
    }
}

impl Default for AppState {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// EDGE SYNC TRAIT
// ============================================================================

/// Trait for syncing state to edge (Cloudflare Durable Objects)
#[async_trait]
pub trait EdgeSync: Send + Sync {
    /// Sync state to edge
    async fn sync_to_edge(&self, state: &AppState) -> Result<(), SyncError>;

    /// Pull state from edge
    async fn sync_from_edge(&self) -> Result<AppState, SyncError>;

    /// Subscribe to edge updates (WebSocket)
    async fn subscribe_edge_updates(&self) -> Result<Box<dyn EdgeUpdateStream>, SyncError>;
}

/// Stream of edge updates
#[async_trait]
pub trait EdgeUpdateStream: Send + Sync {
    /// Receive next update
    async fn recv(&mut self) -> Result<AppState, SyncError>;

    /// Check if stream is closed
    fn is_closed(&self) -> bool;
}

// ============================================================================
// LOCAL STORAGE TRAIT
// ============================================================================

/// Trait for persisting state locally (IndexedDB in browser)
#[async_trait]
pub trait LocalStorage: Send + Sync {
    /// Save state to local storage
    async fn save(&self, state: &AppState) -> Result<(), SyncError>;

    /// Load state from local storage
    async fn load(&self) -> Result<Option<AppState>, SyncError>;

    /// Clear local storage
    async fn clear(&self) -> Result<(), SyncError>;
}

// ============================================================================
// STATE MERGE STRATEGIES
// ============================================================================

/// Strategy for merging conflicting edits
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MergeStrategy {
    /// Last write wins (timestamp-based)
    LastWriteWins,
    /// First write wins
    FirstWriteWins,
    /// Manual resolution required
    Manual,
    /// Automatic merge (combine if possible)
    AutoMerge,
}

/// Merge options for state operations
pub struct MergeOptions {
    pub strategy: MergeStrategy,
    pub conflict_resolution: Option<ConflictResolution>,
}

/// How to resolve merge conflicts
pub enum ConflictResolution {
    /// Use local version
    UseLocal,
    /// Use remote version
    UseRemote,
    /// Custom merge function
    Custom(Box<dyn Fn(AppState, AppState) -> AppState + Send + Sync>),
}

// ============================================================================
// STATE SERIALIZATION
// ============================================================================

/// Serialize state to JSON for storage/transmission
pub fn serialize_state(state: &AppState) -> Result<Vec<u8>, SyncError> {
    serde_json::to_vec(state)
        .map_err(|e| SyncError::Serialization(e.to_string()))
}

/// Deserialize state from JSON
pub fn deserialize_state(data: &[u8]) -> Result<AppState, SyncError> {
    serde_json::from_slice(data)
        .map_err(|e| SyncError::Serialization(e.to_string()))
}

/// Export state as pretty JSON (for debugging)
pub fn export_state_json(state: &AppState) -> Result<String, SyncError> {
    serde_json::to_string_pretty(state)
        .map_err(|e| SyncError::Serialization(e.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use web_nexus_contracts::{ShowStatus, Role, UserStatus};

    #[test]
    fn test_state_creation() {
        let state = AppState::new();
        assert!(matches!(state.sync_status, SyncStatus::Synced));
        assert_eq!(state.clock, 0);
    }

    #[test]
    fn test_add_show() {
        let mut state = AppState::new();
        let show = Show {
            id: "show-1".to_string(),
            site_id: "site-1".to_string(),
            title: "Test Show".to_string(),
            venue: "Test Venue".to_string(),
            address: None,
            date: chrono::Utc::now().timestamp() + 86400,
            start_time: "21:00".to_string(),
            ticket_url: None,
            description: None,
            status: ShowStatus::Upcoming,
            created_by: "user-1".to_string(),
            created_at: chrono::Utc::now().timestamp(),
            updated_at: chrono::Utc::now().timestamp(),
        };

        state.add_show(show).unwrap();
        assert!(state.needs_sync());
        assert_eq!(state.clock, 1);

        let shows = state.get_site_shows("site-1");
        assert_eq!(shows.len(), 1);
        assert_eq!(shows[0].title, "Test Show");
    }

    #[test]
    fn test_state_merge() {
        let mut state1 = AppState::new();
        let mut state2 = AppState::new();

        let show1 = Show {
            id: "show-1".to_string(),
            site_id: "site-1".to_string(),
            title: "Show from State 1".to_string(),
            venue: "Venue 1".to_string(),
            address: None,
            date: chrono::Utc::now().timestamp() + 86400,
            start_time: "21:00".to_string(),
            ticket_url: None,
            description: None,
            status: ShowStatus::Upcoming,
            created_by: "user-1".to_string(),
            created_at: chrono::Utc::now().timestamp(),
            updated_at: 1000,
        };

        let show2 = Show {
            id: "show-2".to_string(),
            site_id: "site-1".to_string(),
            title: "Show from State 2".to_string(),
            venue: "Venue 2".to_string(),
            address: None,
            date: chrono::Utc::now().timestamp() + 86400,
            start_time: "21:00".to_string(),
            ticket_url: None,
            description: None,
            status: ShowStatus::Upcoming,
            created_by: "user-2".to_string(),
            created_at: chrono::Utc::now().timestamp(),
            updated_at: 2000,
        };

        state1.add_show(show1).unwrap();
        state2.add_show(show2).unwrap();

        state1.merge(state2);

        let shows = state1.get_site_shows("site-1");
        assert_eq!(shows.len(), 2);
    }
}
