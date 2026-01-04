// Web Nexus CMS - State Stores
//
// Global state management with Leptos reactive signals

use leptos::prelude::*;
use web_nexus_state::AppState;

/// Global application state store
#[derive(Clone)]
pub struct AppStateStore {
    pub state: RwSignal<AppState>,
}

impl AppStateStore {
    /// Create a new state store
    pub fn new() -> Self {
        Self {
            state: RwSignal::new(AppState::new()),
        }
    }

    /// Get the current state
    pub fn get(&self) -> AppState {
        self.state.get()
    }

    /// Update the state
    pub fn set(&self, new_state: AppState) {
        self.state.set(new_state);
    }
}

impl Default for AppStateStore {
    fn default() -> Self {
        Self::new()
    }
}

/// User authentication store
#[derive(Clone)]
pub struct AuthStore {
    pub is_authenticated: RwSignal<bool>,
    pub user_id: RwSignal<Option<String>>,
}

impl AuthStore {
    /// Create a new auth store
    pub fn new() -> Self {
        Self {
            is_authenticated: RwSignal::new(false),
            user_id: RwSignal::new(None),
        }
    }

    /// Log in a user
    pub fn login(&self, user_id: String) {
        self.user_id.set(Some(user_id));
        self.is_authenticated.set(true);
    }

    /// Log out the current user
    pub fn logout(&self) {
        self.user_id.set(None);
        self.is_authenticated.set(false);
    }
}

impl Default for AuthStore {
    fn default() -> Self {
        Self::new()
    }
}

/// UI state store (modals, loading states, etc.)
#[derive(Clone)]
pub struct UIStore {
    pub is_loading: RwSignal<bool>,
    pub error_message: RwSignal<Option<String>>,
}

impl UIStore {
    /// Create a new UI store
    pub fn new() -> Self {
        Self {
            is_loading: RwSignal::new(false),
            error_message: RwSignal::new(None),
        }
    }

    /// Show loading state
    pub fn set_loading(&self, loading: bool) {
        self.is_loading.set(loading);
    }

    /// Show an error message
    pub fn set_error(&self, message: String) {
        self.error_message.set(Some(message));
    }

    /// Clear error message
    pub fn clear_error(&self) {
        self.error_message.set(None);
    }
}

impl Default for UIStore {
    fn default() -> Self {
        Self::new()
    }
}
