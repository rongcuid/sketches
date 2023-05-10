use sk01::run;
use winit::{event::*, event_loop::*, window::*};

fn main() {
    pollster::block_on(run());
}
