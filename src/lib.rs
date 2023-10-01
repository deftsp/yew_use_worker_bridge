pub mod agent;
use agent::*;
use yew::prelude::*;
use yew_agent::worker::{use_worker_bridge, WorkerProvider};

#[function_component]
fn Main() -> Html {
    let _event_bus_bridge = use_worker_bridge::<EventBusWorker, _>(|_| {});

    let count = use_state_eq(|| 0);

    let add = use_callback(
        count.clone(),
        |_input, count| {
            count.set(**count + 1);
        }
    );

    html! {
        <>
            <h1>{"use_worker_bridge_example"}</h1>
            <button onclick={add}>{ "submit" }</button>
            <p>{*count}</p>
        </>
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <WorkerProvider<EventBusWorker> path="/worker.js">
            <Main />
        </WorkerProvider<EventBusWorker>>
    }
}
