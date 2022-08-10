use dioxus::prelude::*;
use dioxus_free_icons::icons::io_icons::{IoPauseSharp, IoPlaySharp};
use dioxus_free_icons::Icon;

extern crate anyhow;
extern crate cpal;
use crate::player::{Player, PlayerTrait};

#[allow(non_snake_case)]
pub fn App(cx: Scope) -> Element {
    let is_playing = use_state(&cx, || true);
    let player = use_ref(&cx, Player::new);

    let play_and_pause_button = match is_playing.current().as_ref() {
        true => rsx!(button {
            onclick: move |_| {
                is_playing.set(false);
                player.write_silent().pause();
            },

            Icon {
                width: 50,
                height: 50,
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
                width: 50,
                height: 50,
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

    cx.render(rsx!(
        link { href: "/src/style.css", rel: "stylesheet" }
        div { class: "h-screen bg-zinc-800 flex flex-col justify-center items-center",
            div {
                class: "flex",

                play_and_pause_button
            }
        }
    ))
}