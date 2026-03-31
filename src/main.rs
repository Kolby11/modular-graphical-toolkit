mod markup;
mod platform;
mod renderer;
mod widget;
mod layout;

use platform::WaylandPlatform;

fn main() {
    env_logger::init();

    let (mut platform, mut event_queue) =
        WaylandPlatform::new().expect("Failed to connect to Wayland compositor");

    platform.run(&mut event_queue);
}
