mod player;
mod protocol;
mod protodef;
mod test;

use player::*;

use rust_raknet::RaknetListener;

#[tokio::main]
async fn main() {
    let mut listener = RaknetListener::bind(&"0.0.0.0:19132".parse().unwrap())
        .await
        .unwrap();
    listener
        .set_motd("rust-raknet", 20, "568", "1.19.62", "Survival", 19132)
        .await;
    listener.listen().await;
    while let Ok(socket) = listener.accept().await {
        tokio::spawn(async move {
            let player = Player::new(socket);
            player.listen().await;
        });
    }
}
