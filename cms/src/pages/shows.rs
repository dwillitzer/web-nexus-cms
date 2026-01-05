// Web Nexus CMS - Shows Management Page
//
// CRUD interface for shows

use leptos::prelude::*;
use leptos::either::Either;
use leptos_router::components::Redirect;
use crate::stores::AuthStore;
use crate::components::{Layout, Card, Button, Table, Input};

#[derive(Debug, Clone, PartialEq)]
struct Show {
    id: String,
    venue: String,
    city: String,
    date: String,
    status: String,
}

/// Shows list page component
#[component]
pub fn ShowsPage(auth_store: AuthStore) -> impl IntoView {
    let is_authenticated = auth_store.is_authenticated;

    // Redirect if not authenticated
    let redirect = Signal::derive(move || {
        if !is_authenticated.get() {
            Some("/login".to_string())
        } else {
            None
        }
    });

    // Mock shows data
    let shows = RwSignal::new(vec![
        Show {
            id: "1".to_string(),
            venue: "The Blue Note".to_string(),
            city: "New York, NY".to_string(),
            date: "2025-02-15".to_string(),
            status: "Upcoming".to_string(),
        },
        Show {
            id: "2".to_string(),
            venue: "Paradise Rock Club".to_string(),
            city: "Boston, MA".to_string(),
            date: "2025-02-22".to_string(),
            status: "Upcoming".to_string(),
        },
        Show {
            id: "3".to_string(),
            venue: "9:30 Club".to_string(),
            city: "Washington, DC".to_string(),
            date: "2025-03-01".to_string(),
            status: "Upcoming".to_string(),
        },
    ]);

    let show_form = RwSignal::new(None::<Show>);
    let is_editing = RwSignal::new(false);

    let handle_new_show = Callback::new(move |_| {
        show_form.set(Some(Show {
            id: uuid::Uuid::new_v4().to_string(),
            venue: String::new(),
            city: String::new(),
            date: String::new(),
            status: "Upcoming".to_string(),
        }));
        is_editing.set(true);
    });

    let handle_edit_show = Callback::new({
        let shows = shows.clone();
        let show_form = show_form.clone();
        let is_editing = is_editing.clone();
        move |id: String| {
            let show = shows.get().into_iter().find(|s| s.id == id).unwrap();
            show_form.set(Some(show));
            is_editing.set(true);
        }
    });

    let handle_delete_show = Callback::new({
        let shows = shows.clone();
        move |id: String| {
            shows.update(|s| s.retain(|show| show.id != id));
        }
    });

    let handle_save_show = Callback::new({
        let shows = shows.clone();
        let show_form = show_form.clone();
        let is_editing = is_editing.clone();
        move |_| {
            if let Some(show) = show_form.get() {
                shows.update(|s| {
                    if let Some(existing) = s.iter().position(|x| x.id == show.id) {
                        s[existing] = show.clone();
                    } else {
                        s.push(show.clone());
                    }
                });
                show_form.set(None);
                is_editing.set(false);
            }
        }
    });

    let handle_cancel = Callback::new(move |_| {
        show_form.set(None);
        is_editing.set(false);
    });

    view! {
        {move || {
            redirect.get().map(|path| view! {
                <Redirect path=path />
            })
        }}

        <Layout title="Shows".to_string() auth_store=auth_store>
            <div class="shows-page">
                {move || {
                    if is_editing.get() {
                        Either::Left(view! {
                            <div class="show-form-wrapper">
                                <Card title=Some("New Show".to_string())>
                                    <ShowForm
                                        show=show_form
                                        on_save=handle_save_show
                                        on_cancel=handle_cancel
                                    />
                                </Card>
                            </div>
                        })
                    } else {
                        Either::Right(view! {
                            <div class="shows-list">
                                <div class="page-actions">
                                    <Button
                                        label="New Show".to_string()
                                        on_click=Some(handle_new_show)
                                        variant=None
                                    />
                                </div>

                                <Card title=None>
                                    <Table headers=vec![
                                        "Date".to_string(),
                                        "Venue".to_string(),
                                        "Location".to_string(),
                                        "Status".to_string(),
                                        "Actions".to_string(),
                                    ]>
                                        {move || {
                                            shows.get().into_iter().map(|show| {
                                                let show_clone = show.clone();
                                                view! {
                                                    <tr>
                                                        <td>{show_clone.date.clone()}</td>
                                                        <td>{show_clone.venue.clone()}</td>
                                                        <td>{show_clone.city.clone()}</td>
                                                        <td>{show_clone.status.clone()}</td>
                                                        <td class="actions">
                                                            <Button
                                                                label="Edit".to_string()
                                                                variant=Some("secondary".to_string())
                                                                on_click=Some(Callback::new({
                                                                    let id = show_clone.id.clone();
                                                                    move |_| handle_edit_show.run(id.clone())
                                                                }))
                                                            />
                                                            <Button
                                                                label="Delete".to_string()
                                                                variant=Some("danger".to_string())
                                                                on_click=Some(Callback::new({
                                                                    let id = show_clone.id.clone();
                                                                    move |_| handle_delete_show.run(id.clone())
                                                                }))
                                                            />
                                                        </td>
                                                    </tr>
                                                }
                                            }).collect::<Vec<_>>()
                                        }}
                                    </Table>
                                </Card>
                            </div>
                        })
                    }
                }}
            </div>
        </Layout>
    }
}

/// Show form component
#[component]
fn ShowForm(
    show: RwSignal<Option<Show>>,
    on_save: Callback<leptos::ev::MouseEvent>,
    on_cancel: Callback<leptos::ev::MouseEvent>,
) -> impl IntoView {
    let current_show = Signal::derive(move || show.get().unwrap_or_else(|| Show {
        id: uuid::Uuid::new_v4().to_string(),
        venue: String::new(),
        city: String::new(),
        date: String::new(),
        status: "Upcoming".to_string(),
    }));

    let venue = RwSignal::new(current_show.get_untracked().venue);
    let city = RwSignal::new(current_show.get_untracked().city);
    let date = RwSignal::new(current_show.get_untracked().date);

    view! {
        <form class="show-form" on:submit=|e| e.prevent_default()>
            <Input
                label="Venue".to_string()
                name="venue".to_string()
                placeholder=Some("Enter venue name".to_string())
                value=venue
                required=true
            />

            <Input
                label="City".to_string()
                name="city".to_string()
                placeholder=Some("Enter city and state".to_string())
                value=city
                required=true
            />

            <Input
                label="Date".to_string()
                name="date".to_string()
                input_type="date".to_string()
                value=date
                required=true
            />

            <div class="form-actions">
                <Button
                    label="Save".to_string()
                    on_click=Some(on_save)
                    variant=None
                />
                <Button
                    label="Cancel".to_string()
                    on_click=Some(on_cancel)
                    variant=Some("secondary".to_string())
                />
            </div>
        </form>
    }
}
