mod player;

use player::{Player, PlayerMsg};
use std::{
    sync::mpsc::{self, Receiver, Sender},
    thread, time,
};

fn main() {
    let (message_tx, _): (Sender<PlayerMsg>, Receiver<PlayerMsg>) = mpsc::channel();
    let mut player = Player::new(message_tx);
    player.play("examples/square_120bpm_4bars_16bit_44100hz.wav");
    thread::sleep(time::Duration::from_secs(10));
}
