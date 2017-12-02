extern crate cgmath;
extern crate midgar;

mod app;

fn main() {
    let app_config = midgar::MidgarAppConfig::new()
        .with_title("Cat Herding")
        .with_screen_size((1280, 800));
    let app: midgar::MidgarApp<app::GameApp> = midgar::MidgarApp::new(app_config);
    app.run();
}
