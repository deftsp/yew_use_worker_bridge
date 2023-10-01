// -*- coding: utf-8-unix; -*-

use std::collections::HashSet;
use serde::{Deserialize, Serialize};
use yew_agent::worker::{HandlerId, WorkerScope};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum EventBusMessage {
    WindowResize,
    Nope,
}

pub struct EventBusWorker {
    subscribers: HashSet<HandlerId>,
}

impl yew_agent::worker::Worker for EventBusWorker {
    type Input = EventBusMessage;
    type Output = EventBusMessage;
    type Message = ();

    fn create(_scope: &WorkerScope<Self>) -> Self {
        EventBusWorker { subscribers: Default::default() }
    }

    fn update(&mut self, _scope: &WorkerScope<Self>, _msg: Self::Message) {
        // do nothing
    }

    fn connected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        log::debug!("agent/event_bus connected: {id:?}");
        self.subscribers.insert(id);
    }

    fn disconnected(&mut self, _scope: &WorkerScope<Self>, id: HandlerId) {
        log::debug!("agent/event_bus disconnected: {id:?}");
        self.subscribers.remove(&id);
    }

    fn received(&mut self, scope: &WorkerScope<Self>, msg: Self::Input, _id: HandlerId) {
        // forward the input to output
        log::debug!("agent/event_bus: {msg:?}");
        // NOTE: might do not send to the bridge which send the message
        self.subscribers.iter().for_each(|id| {
            scope.respond(*id, msg.clone());
        })
    }
}
