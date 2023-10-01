fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    yew::Renderer::<yew_use_worker_bridge::App>::new().render();
}
