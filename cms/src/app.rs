// Web Nexus CMS - Main App Component (Leptos)
//
// Root application component with routing and layout

use leptos::prelude::*;

/// Main App component - root of the CMS admin portal
#[component]
pub fn App() -> impl IntoView {
    view! {
        <div class="cms-app">
            <h1>"Web Nexus CMS"</h1>
            <p>"Admin Portal - Leptos WASM Application"</p>
            <p class="stub-notice">"NOTE: This is a stub implementation pending full development"</p>
        </div>
    }
}
