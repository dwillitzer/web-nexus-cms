// Web Nexus CMS - Login Page
//
// User authentication page

use leptos::prelude::*;
use leptos_router::components::Redirect;
use crate::stores::AuthStore;

/// Login page component
#[component]
pub fn LoginPage(auth_store: AuthStore) -> impl IntoView {
    let email = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let is_loading = RwSignal::new(false);
    let error_message = RwSignal::new(None::<String>);

    let is_authenticated = auth_store.is_authenticated;

    // Redirect if already authenticated
    let redirect = Signal::derive(move || {
        if is_authenticated.get() {
            Some("/".to_string())
        } else {
            None
        }
    });

    let handle_login = {
        let email = email.clone();
        let password = password.clone();
        let is_loading = is_loading.clone();
        let error_message = error_message.clone();
        let auth_store = auth_store.clone();

        move |_| {
            // Validate inputs
            if email.get().is_empty() || password.get().is_empty() {
                error_message.set(Some("Please enter email and password".to_string()));
                return;
            }

            is_loading.set(true);
            error_message.set(None);

            // TODO: Make API call to /api/auth/login
            // For now, simulate login with a hardcoded user
            // Simulate API call with set_timeout
            let auth_store_clone = auth_store.clone();
            let is_loading_clone = is_loading.clone();
            let error_message_clone = error_message.clone();
            let email_clone = email.clone();

            // Simulate API call - for now just synchronous mock
            // In production, this would be a real API call
            if email_clone.get() == "admin@example.com" {
                auth_store_clone.login("user-123".to_string());
                is_loading_clone.set(false);
            } else {
                error_message_clone.set(Some("Invalid credentials".to_string()));
                is_loading_clone.set(false);
            }
        }
    };

    view! {
        {move || {
            redirect.get().map(|path| view! {
                <Redirect path=path />
            })
        }}

        <div class="login-page">
            <div class="login-container">
                <div class="login-card">
                    <div class="login-header">
                        <h1>"Web Nexus CMS"</h1>
                        <p>"Admin Portal Login"</p>
                    </div>

                    {move || {
                        error_message.get().map(|msg| view! {
                            <div class="alert alert-error">{msg}</div>
                        })
                    }}

                    <form on:submit=move |e| {
                        e.prevent_default();
                        handle_login(e);
                    }>
                        <div class="form-group">
                            <label for="email">"Email"</label>
                            <input
                                type="email"
                                id="email"
                                prop:value=move || email.get()
                                on:input=move |e| {
                                    email.set(event_target_value(&e));
                                }
                                placeholder="admin@example.com"
                                required=true
                            />
                        </div>

                        <div class="form-group">
                            <label for="password">"Password"</label>
                            <input
                                type="password"
                                id="password"
                                prop:value=move || password.get()
                                on:input=move |e| {
                                    password.set(event_target_value(&e));
                                }
                                placeholder="••••••••"
                                required=true
                            />
                        </div>

                        <button
                            type="submit"
                            class="btn btn-primary btn-block"
                            disabled=move || is_loading.get()
                        >
                            {move || if is_loading.get() { "Logging in..." } else { "Login" }}
                        </button>
                    </form>

                    <div class="login-footer">
                        <p>"Demo: admin@example.com"</p>
                    </div>
                </div>
            </div>
        </div>
    }
}
