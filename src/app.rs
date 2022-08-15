use std::time::Duration;

use dioxus::desktop::use_window;
use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{IoPauseSharp, IoPlaySharp};
use dioxus_free_icons::Icon;
use tokio::time;

extern crate anyhow;
extern crate cpal;
use crate::player::{self, PlayerTrait};

fn format_time(time: u64) -> String {
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

    let elapsed_format = use_state(&cx, || "0:00".to_string());
    let duration_format = use_state(&cx, || "0:00".to_string());
    let progress = use_state(&cx, || 0.);
    
    use_future(&cx, (interval, player, elapsed_format, duration_format, progress), |(interval, player, elapsed_format, duration_format, progress)| async move {
        loop {
            interval.write_silent().tick().await;
            let elapsed = player.read().elapsed().as_secs();
            let duration = player.read().duration().unwrap() as u64;
            elapsed_format.set(format_time(elapsed));
            duration_format.set(format_time(duration));
            progress.set(elapsed as f64 / duration as f64 * 100.);
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
                    class: "flex-1 mx-3 py-2 group relative w-auto",

                    div {
                        class: "rounded-full w-full h-1 bg-zinc-500 absolute top-0 bottom-0 left-0 right-0 m-auto"
                    }
                    div {
                        width: "{progress}%",
                        class: "rounded-full h-1 bg-zinc-300 group-hover:bg-sky-500 absolute top-0 bottom-0 left-0 right-0 my-auto"
                    }
                    div {
                        left: "{progress}%",
                        class: "rounded-full w-4 h-4 bg-zinc-300 opacity-0 group-hover:opacity-100 absolute top-0 bottom-0 my-auto -translate-x-[50%]"
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
