use actix_latest::MessageApp;

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let app = MessageApp::new("127.0.0.1", 8080);
    app.run()
}
