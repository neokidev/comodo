use std::time::Duration;

use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{IoPauseSharp, IoPlaySharp, IoBug};
use dioxus_free_icons::Icon;
use tokio::time;

extern crate anyhow;
extern crate cpal;
use crate::player::{self, PlayerTrait};

fn format_time(time: &u64) -> String {
    let min = time / 60;
    let sec = time % 60;
    format!("{}:{:02}", min, sec)
}

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    let is_playing = use_state(&cx, || true);
    let player = use_ref(&cx, player::try_open);
    let interval = use_ref(&cx, || time::interval(Duration::from_millis(1)));

    let window = use_window(&cx);

    let play_and_pause_button = match is_playing.current().as_ref() {
        true => rsx!(button {
            onclick: move |_| {
                is_playing.set(false);
                player.write_silent().pause();
            },

            Icon {
                width: 40,
                height: 40,
                fill: "white",
                icon: IoPauseSharp,
            }
        }),
        false => rsx!(button {
            onclick: move |_| {
                is_playing.set(true);
                player.write_silent().resume();
            },

            Icon {
                width: 40,
                height: 40,
                fill: "white",
                icon: IoPlaySharp,
            }
        }),
    };

    use_future(&cx, (player,), |(player,)| async move {
        player
            .write_silent()
            .add_and_play("./examples/square_120bpm_4bars_16bit_44100hz.wav");
    });

    let elapsed = use_state(&cx, || 0);
    let elapsed_format = format_time(elapsed.get());
    let duration = use_state(&cx, || 0);
    let duration_format = format_time(duration.get());
    let progress = use_state(&cx, || 0.);
    let is_dragging_progress_bar = use_state(&cx, || false);
    
    use_future(&cx, (interval, player, elapsed, duration, progress, is_dragging_progress_bar), |(interval, player, elapsed, duration, progress, is_dragging_progress_bar)| async move {
        loop {
            interval.write_silent().tick().await;

            if !*is_dragging_progress_bar.get() {
                let e = player.read().elapsed().as_secs();
                let d = player.read().duration().unwrap() as u64;
                elapsed.set(e);
                duration.set(d);
                progress.set(e as f64 / d as f64 * 100.);
            }
        }
    });

    cx.render(rsx!(
        link { href: "/src/style.css", rel: "stylesheet" }
        div { class: "h-screen bg-zinc-800 flex flex-col justify-center items-center",
            div {
                class: "w-full px-6 flex items-center",
                
                div {
                    class: "w-8 text-gray-500 text-xs font-mono",

                    div {
                        class: "text-right overflow-hidden",
                        "{elapsed_format}"
                    }
                }

                div {
                    class: "flex-1 mx-3 my-2 group relative w-auto",

                    div {
                        class: "py-2",
                        onmousedown: move |evt| {
                            let x = evt.client_x - (24 + 32 + 12);
                            let progress_bar_width = 480 - (24 + 32 + 12) * 2;

                            let d = *duration.get() as f64;
                            let e = x as f64 / progress_bar_width as f64 * d;

                            elapsed.set(e as u64);
                            progress.set(e / d * 100.);
                            is_dragging_progress_bar.set(true);
                        },
                        onmousemove: move |evt| {
                            if *is_dragging_progress_bar.get() {
                                let x = evt.client_x - (24 + 32 + 12);
                                let progress_bar_width = 480 - (24 + 32 + 12) * 2;

                                let d = *duration.get() as f64;
                                let e = x as f64 / progress_bar_width as f64 * d;

                                elapsed.set(e as u64);
                                progress.set(e / d * 100.);
                            }
                        },
                        onmouseup: move |evt| {
                            let x = evt.client_x - (24 + 32 + 12);
                            let progress_bar_width = 480 - (24 + 32 + 12) * 2;

                            let d = *duration.get() as f64;
                            let e = x as f64 / progress_bar_width as f64 * d;

                            elapsed.set(e as u64);
                            progress.set(e / d * 100.);

                            player.write_silent().seek(e as i64);

                            is_dragging_progress_bar.set(false);
                        },
                        div {
                            class: "rounded-full w-full h-1 bg-zinc-500 absolute top-0 bottom-0 left-0 right-0 m-auto"
                        }
                    }
                    div {
                        width: "{progress}%",
                        class: "rounded-full h-1 bg-zinc-300 group-hover:bg-sky-500 absolute top-0 bottom-0 left-0 right-0 my-auto pointer-events-none"
                    }
                    div {
                        left: "{progress}%",
                        class: "rounded-full w-4 h-4 bg-zinc-300 opacity-0 group-hover:opacity-100 absolute top-0 bottom-0 my-auto -translate-x-[50%] pointer-events-none"
                    }
                }

                div {
                    class: "w-8 text-gray-500 text-xs font-mono",

                    div {
                        class: "text-right overflow-hidden",
                        "{duration_format}"
                    }
                }
            }
                
            div {
                class: "flex mt-2",
                play_and_pause_button
            }

            div {
                class: "w-full px-4 flex justify-end",

                button {
                    onclick: move |_| window.devtool(),

                    Icon {
                        width: 20,
                        height: 20,
                        fill: "gray",
                        icon: IoBug,
                    }
                }
            }
        }
    ))
}
