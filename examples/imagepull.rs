// cargo run --example imagepull busybox

use shiplift::{Docker, PullOptions};
use std::env;
use tokio::prelude::{Future, Stream};

fn main() {
    env_logger::init();
    let docker = Docker::new();
    let img = env::args()
        .nth(1)
        .expect("You need to specify an image name");
    let fut = docker
        .images()
        .pull(&PullOptions::builder().image(img).build())
        .for_each(|output| {
            println!("{:?}", output);
            Ok(())
        })
        .map_err(|e| eprintln!("Error: {}", e));
    tokio::run(fut);
}
