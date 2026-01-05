// Web Nexus CMS - Reusable Components
//
// UI components for the admin portal

use leptos::prelude::*;
use leptos_router::components::*;
use crate::stores::AuthStore;

/// Header component for CMS pages
#[component]
pub fn Header(title: String, auth_store: AuthStore) -> impl IntoView {
    let user_id = auth_store.user_id;
    let logout = {
        let auth_store = auth_store.clone();
        move |_| {
            auth_store.logout();
        }
    };

    view! {
        <header class="cms-header">
            <div class="header-left">
                <h1>{title}</h1>
            </div>
            <div class="header-right">
                {move || {
                    user_id.get().map(|uid| view! {
                        <span class="user-info">{uid}</span>
                    })
                }}
                <button on:click=logout class="btn-logout">"Logout"</button>
            </div>
        </header>
    }
}

/// Sidebar navigation component
#[component]
pub fn Sidebar() -> impl IntoView {
    view! {
        <nav class="cms-sidebar">
            <div class="sidebar-brand">
                <h2>"Web Nexus"</h2>
            </div>
            <ul class="sidebar-nav">
                <li>
                    <A href="/">"Dashboard"</A>
                </li>
                <li>
                    <A href="/shows">"Shows"</A>
                </li>
                <li>
                    <A href="/songs">"Songs"</A>
                </li>
                <li>
                    <A href="/posts">"Blog Posts"</A>
                </li>
                <li>
                    <A href="/photos">"Photos"</A>
                </li>
                <li>
                    <A href="/videos">"Videos"</A>
                </li>
            </ul>
        </nav>
    }
}

/// Main layout component
#[component]
pub fn Layout(
    title: String,
    auth_store: AuthStore,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="cms-layout">
            <Sidebar />
            <div class="cms-main">
                <Header title=title auth_store=auth_store />
                <main class="cms-content">
                    {children()}
                </main>
            </div>
        </div>
    }
}

/// Loading spinner component
#[component]
pub fn Loading() -> impl IntoView {
    view! {
        <div class="loading-spinner">
            <div class="spinner"></div>
            <p>"Loading..."</p>
        </div>
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

/// Card component for content grouping
#[component]
pub fn Card(
    title: Option<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="card">
            {move || {
                title.as_ref().map(|t| view! {
                    <div class="card-header">
                        <h3>{t.clone()}</h3>
                    </div>
                })
            }}
            <div class="card-body">
                {children()}
            </div>
        </div>
    }
}

/// Button component with variants
#[component]
pub fn Button(
    label: String,
    on_click: Option<Callback<leptos::ev::MouseEvent>>,
    variant: Option<String>,
    #[prop(default = false)] disabled: bool,
) -> impl IntoView {
    let variant = variant.unwrap_or_else(|| "primary".to_string());
    let class = format!("btn btn-{}", variant);

    view! {
        <button
            class=class
            on:click=move |e| {
                if let Some(handler) = &on_click {
                    handler.run(e);
                }
            }
            disabled=disabled
        >
            {label}
        </button>
    }
}

/// Form input component
#[component]
pub fn Input(
    label: String,
    name: String,
    #[prop(default = "text".to_string())] input_type: String,
    #[prop(default = None)] placeholder: Option<String>,
    value: RwSignal<String>,
    #[prop(default = false)] required: bool,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label for=name.clone()>{label}</label>
            <input
                type=input_type
                name=name
                placeholder=placeholder.unwrap_or_default()
                prop:value=move || value.get()
                on:input=move |e| {
                    value.set(event_target_value(&e));
                }
                required=required
            />
        </div>
    }
}

/// Textarea component
#[component]
pub fn Textarea(
    label: String,
    name: String,
    #[prop(default = None)] placeholder: Option<String>,
    value: RwSignal<String>,
    #[prop(default = None)] rows: Option<i32>,
) -> impl IntoView {
    view! {
        <div class="form-group">
            <label for=name.clone()>{label}</label>
            <textarea
                name=name
                placeholder=placeholder.unwrap_or_default()
                prop:value=move || value.get()
                on:input=move |e| {
                    value.set(event_target_value(&e));
                }
                rows=rows.unwrap_or(4)
            ></textarea>
        </div>
    }
}

/// Table component for displaying data
#[component]
pub fn Table(
    headers: Vec<String>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="table-container">
            <table class="data-table">
                <thead>
                    <tr>
                        {headers.into_iter().map(|h| view! {
                            <th>{h}</th>
                        }).collect::<Vec<_>>()}
                    </tr>
                </thead>
                <tbody>
                    {children()}
                </tbody>
            </table>
        </div>
    }
}
