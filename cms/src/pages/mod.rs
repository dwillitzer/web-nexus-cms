// Web Nexus CMS - Page Components
//
// Individual pages for the admin portal

use leptos::prelude::*;
use crate::components::Header;

/// Dashboard page - overview of site status
#[component]
pub fn DashboardPage() -> impl IntoView {
    view! {
        <div class="dashboard-page">
            <Header title="Dashboard".to_string()/>
            <p>"Site overview and statistics"</p>
        </div>
    }
}

/// Shows management page
#[component]
pub fn ShowsPage() -> impl IntoView {
    view! {
        <div class="shows-page">
            <Header title="Shows".to_string()/>
            <p>"Manage shows and setlists"</p>
        </div>
    }
}

/// Content management page
#[component]
pub fn ContentPage() -> impl IntoView {
    view! {
        <div class="content-page">
            <Header title="Content".to_string()/>
            <p>"Manage blog posts, photos, videos"</p>
        </div>
    }
}

/// Settings page
#[component]
pub fn SettingsPage() -> impl IntoView {
    view! {
        <div class="settings-page">
            <Header title="Settings".to_string()/>
            <p>"Site configuration and preferences"</p>
        </div>
    }
}
