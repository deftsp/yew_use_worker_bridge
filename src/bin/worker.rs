use yew_agent::Registrable;
use yew_use_worker_bridge::agent::EventBusWorker;

fn main() {
    use log::Level;
    console_log::init_with_level(Level::Trace).expect("error initializing log");

    EventBusWorker::registrar().register();
}
