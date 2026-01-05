// Web Nexus CMS - Dashboard Page
//
// Overview of site status and statistics

use leptos::prelude::*;
use leptos_router::components::Redirect;
use crate::stores::AuthStore;
use crate::components::{Layout, Card, Button};

/// Dashboard page component
#[component]
pub fn DashboardPage(auth_store: AuthStore) -> impl IntoView {
    let is_authenticated = auth_store.is_authenticated;

    // Redirect if not authenticated
    let redirect = Signal::derive(move || {
        if !is_authenticated.get() {
            Some("/login".to_string())
        } else {
            None
        }
    });

    view! {
        {move || {
            redirect.get().map(|path| view! {
                <Redirect path=path />
            })
        }}

        <Layout title="Dashboard".to_string() auth_store=auth_store>
            <div class="dashboard">
                <div class="stats-grid">
                    <Card title=Some("Total Shows".to_string())>
                        <div class="stat-value">24</div>
                        <div class="stat-label">"Upcoming shows"</div>
                    </Card>

                    <Card title=Some("Songs".to_string())>
                        <div class="stat-value">42</div>
                        <div class="stat-label">"In repertoire"</div>
                    </Card>

                    <Card title=Some("Blog Posts".to_string())>
                        <div class="stat-value">18</div>
                        <div class="stat-label">"Published posts"</div>
                    </Card>

                    <Card title=Some("Photos".to_string())>
                        <div class="stat-value">156</div>
                        <div class="stat-label">"Gallery images"</div>
                    </Card>
                </div>

                <div class="dashboard-grid">
                    <Card title=Some("Recent Activity".to_string())>
                        <ul class="activity-list">
                            <li>
                                <span class="activity-icon">"📝"</span>
                                <div class="activity-content">
                                    <strong>"New blog post"</strong>
                                    <p>"'Summer Tour 2025' was published"</p>
                                </div>
                                <span class="activity-time">"2h ago"</span>
                            </li>
                            <li>
                                <span class="activity-icon">"🎸"</span>
                                <div class="activity-content">
                                    <strong>"Show added"</strong>
                                    <p>"New show at The Blue Note"</p>
                                </div>
                                <span class="activity-time">"5h ago"</span>
                            </li>
                            <li>
                                <span class="activity-icon">"📷"</span>
                                <div class="activity-content">
                                    <strong>"Photos uploaded"</strong>
                                    <p>"12 new photos added to gallery"</p>
                                </div>
                                <span class="activity-time">"1d ago"</span>
                            </li>
                        </ul>
                    </Card>

                    <Card title=Some("Quick Actions".to_string())>
                        <div class="quick-actions">
                            <Button label="New Show".to_string() on_click=None variant=None />
                            <Button label="New Post".to_string() on_click=None variant=None />
                            <Button label="Upload Photos".to_string() on_click=None variant=None />
                            <Button label="Add Song".to_string() on_click=None variant=None />
                        </div>
                    </Card>
                </div>

                <Card title=Some("Upcoming Shows".to_string())>
                    <table class="table-simple">
                        <thead>
                            <tr>
                                <th>"Date"</th>
                                <th>"Venue"</th>
                                <th>"Location"</th>
                                <th>"Actions"</th>
                            </tr>
                        </thead>
                        <tbody>
                            <tr>
                                <td>"2025-02-15"</td>
                                <td>"The Blue Note"</td>
                                <td>"New York, NY"</td>
                                <td>
                                    <Button label="Edit".to_string() on_click=None variant=None />
                                </td>
                            </tr>
                            <tr>
                                <td>"2025-02-22"</td>
                                <td>"Paradise Rock Club"</td>
                                <td>"Boston, MA"</td>
                                <td>
                                    <Button label="Edit".to_string() on_click=None variant=None />
                                </td>
                            </tr>
                            <tr>
                                <td>"2025-03-01"</td>
                                <td>"9:30 Club"</td>
                                <td>"Washington, DC"</td>
                                <td>
                                    <Button label="Edit".to_string() on_click=None variant=None />
                                </td>
                            </tr>
                        </tbody>
                    </table>
                </Card>
            </div>
        </Layout>
    }
}
