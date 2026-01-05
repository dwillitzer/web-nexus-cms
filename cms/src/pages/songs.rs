// Web Nexus CMS - Songs Management Page
//
// Manage song repertoire

use leptos::prelude::*;
use leptos::either::Either;
use leptos_router::components::Redirect;
use crate::stores::AuthStore;
use crate::components::{Layout, Card, Button, Table, Input};

#[derive(Debug, Clone, PartialEq)]
struct Song {
    id: String,
    title: String,
    artist: String,
    duration: Option<String>,
    is_original: bool,
}

/// Songs list page component
#[component]
pub fn SongsPage(auth_store: AuthStore) -> impl IntoView {
    let is_authenticated = auth_store.is_authenticated;

    // Redirect if not authenticated
    let redirect = Signal::derive(move || {
        if !is_authenticated.get() {
            Some("/login".to_string())
        } else {
            None
        }
    });

    // Mock songs data
    let songs = RwSignal::new(vec![
        Song {
            id: "1".to_string(),
            title: "Midnight Train".to_string(),
            artist: "Mike and the Monsters".to_string(),
            duration: Some("4:32".to_string()),
            is_original: true,
        },
        Song {
            id: "2".to_string(),
            title: "Neon Dreams".to_string(),
            artist: "Mike and the Monsters".to_string(),
            duration: Some("3:45".to_string()),
            is_original: true,
        },
        Song {
            id: "3".to_string(),
            title: "Hotel California".to_string(),
            artist: "Eagles".to_string(),
            duration: Some("6:30".to_string()),
            is_original: false,
        },
    ]);

    let show_form = RwSignal::new(false);
    let new_title = RwSignal::new(String::new());
    let new_artist = RwSignal::new(String::new());
    let new_duration = RwSignal::new(String::new());
    let new_is_original = RwSignal::new(true);

    let handle_new_song = Callback::new(move |_| {
        show_form.set(true);
    });

    let handle_save_song = Callback::new({
        let songs = songs.clone();
        let show_form = show_form.clone();
        let new_title = new_title.clone();
        let new_artist = new_artist.clone();
        let new_duration = new_duration.clone();
        let new_is_original = new_is_original.clone();
        move |_| {
            let song = Song {
                id: uuid::Uuid::new_v4().to_string(),
                title: new_title.get(),
                artist: new_artist.get(),
                duration: if new_duration.get().is_empty() { None } else { Some(new_duration.get()) },
                is_original: new_is_original.get(),
            };

            songs.update(|s| s.push(song));
            show_form.set(false);
            new_title.set(String::new());
            new_artist.set(String::new());
            new_duration.set(String::new());
            new_is_original.set(true);
        }
    });

    let handle_cancel = Callback::new({
        let show_form = show_form.clone();
        move |_| {
            show_form.set(false);
        }
    });

    let handle_delete_song = Callback::new({
        let songs = songs.clone();
        move |id: String| {
            songs.update(|s| s.retain(|song| song.id != id));
        }
    });

    view! {
        {move || {
            redirect.get().map(|path| view! {
                <Redirect path=path />
            })
        }}

        <Layout title="Songs".to_string() auth_store=auth_store>
            <div class="songs-page">
                <div class="page-actions">
                    <Button
                        label="Add Song".to_string()
                        on_click=Some(handle_new_song)
                        variant=None
                    />
                </div>

                {move || {
                    if show_form.get() {
                        Either::Left(view! {
                            <Card title=Some("New Song".to_string())>
                                <form class="song-form" on:submit=|e| e.prevent_default()>
                                    <Input
                                        label="Title".to_string()
                                        name="title".to_string()
                                        placeholder=Some("Song title".to_string())
                                        value=new_title
                                        required=true
                                    />

                                    <Input
                                        label="Artist".to_string()
                                        name="artist".to_string()
                                        placeholder=Some("Original artist or cover".to_string())
                                        value=new_artist
                                        required=true
                                    />

                                    <Input
                                        label="Duration".to_string()
                                        name="duration".to_string()
                                        placeholder=Some("4:32".to_string())
                                        value=new_duration
                                    />

                                    <div class="form-group">
                                        <label>
                                            <input
                                                type="checkbox"
                                                prop:checked=move || new_is_original.get()
                                                on:change=move |e| {
                                                    new_is_original.set(event_target_checked(&e));
                                                }
                                            />
                                            " Original song"
                                        </label>
                                    </div>

                                    <div class="form-actions">
                                        <Button
                                            label="Save".to_string()
                                            on_click=Some(handle_save_song)
                                            variant=None
                                        />
                                        <Button
                                            label="Cancel".to_string()
                                            on_click=Some(handle_cancel)
                                            variant=Some("secondary".to_string())
                                        />
                                    </div>
                                </form>
                            </Card>
                        })
                    } else {
                        Either::Right(view! {
                            <Card title=None>
                                <Table headers=vec![
                                    "Title".to_string(),
                                    "Artist".to_string(),
                                    "Duration".to_string(),
                                    "Type".to_string(),
                                    "Actions".to_string(),
                                ]>
                                    {move || {
                                        songs.get().into_iter().map(|song| {
                                            let song_clone = song.clone();
                                            view! {
                                                <tr>
                                                    <td>{song_clone.title.clone()}</td>
                                                    <td>{song_clone.artist.clone()}</td>
                                                    <td>{song_clone.duration.clone().unwrap_or_else(|| "--".to_string())}</td>
                                                    <td>
                                                        {if song_clone.is_original {
                                                            "Original"
                                                        } else {
                                                            "Cover"
                                                        }}
                                                    </td>
                                                    <td class="actions">
                                                        <Button
                                                            label="Delete".to_string()
                                                            variant=Some("danger".to_string())
                                                            on_click=Some(Callback::new({
                                                                let id = song_clone.id.clone();
                                                                move |_| handle_delete_song.run(id.clone())
                                                            }))
                                                        />
                                                    </td>
                                                </tr>
                                            }
                                        }).collect::<Vec<_>>()
                                    }}
                                </Table>
                            </Card>
                        })
                    }
                }}
            </div>
        </Layout>
    }
}
