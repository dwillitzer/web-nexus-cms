// Web Nexus CMS - Main App Component (Leptos)
//
// Root application component with routing and layout

use leptos::prelude::*;
use leptos_router::components::{Router, Routes, Route};
use leptos_router::path;
use crate::stores::{AuthStore, UIStore};
use crate::pages::{LoginPage, DashboardPage, ShowsPage, SongsPage};

/// Main App component - root of the CMS admin portal
#[component]
pub fn App() -> impl IntoView {
    // Provide global stores
    let auth_store = AuthStore::new();
    let _ui_store = UIStore::new();

    // Clone auth_store for each route to avoid move errors
    let auth_store_login = auth_store.clone();
    let auth_store_dashboard = auth_store.clone();
    let auth_store_shows = auth_store.clone();
    let auth_store_songs = auth_store;

    view! {
        <div class="cms-app">
            <Router>
                <Routes fallback=move || {
                    view! { <div>"404 - Page not found"</div> }
                }>
                    // Login route (no layout)
                    <Route path=path!("/login") view=move || {
                        view! { <LoginPage auth_store=auth_store_login.clone() /> }
                    } />

                    // Protected routes with layout
                    <Route path=path!("/") view=move || {
                        view! { <DashboardPage auth_store=auth_store_dashboard.clone() /> }
                    } />
                    <Route path=path!("/shows") view=move || {
                        view! { <ShowsPage auth_store=auth_store_shows.clone() /> }
                    } />
                    <Route path=path!("/songs") view=move || {
                        view! { <SongsPage auth_store=auth_store_songs.clone() /> }
                    } />
                </Routes>
            </Router>
        </div>
    }
}
