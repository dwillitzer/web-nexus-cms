// Web Nexus CMS - Reusable Components
//
// UI components for the admin portal

use leptos::prelude::*;

/// Header component for CMS pages
#[component]
pub fn Header(title: String) -> impl IntoView {
    view! {
        <header class="cms-header">
            <h1>{title}</h1>
        </header>
    }
}

/// Loading spinner component
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading-spinner">"Loading..."</div>
    }
}

/// Error display component
#[component]
pub fn ErrorDisplay(message: String) -> impl IntoView {
    view! {
        <div class="error-display">
            <p>{message}</p>
        </div>
    }
}
