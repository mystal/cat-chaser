extern crate cgmath;
extern crate midgar;
extern crate rand;
extern crate ears;

mod app;
mod config;
mod entities;
mod level;
mod renderer;
mod world;
mod sounds;
mod party;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("Cat Chaser")
        .with_screen_size((config::SCREEN_SIZE.x, config::SCREEN_SIZE.y));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
