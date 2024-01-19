use crate::ws::EventClient;
use common::PayloadMeta;
use macros::generate_state;
use uuid::Uuid;
use yew::prelude::NodeRef;

use std::{ops::DerefMut, sync::Mutex};

use crate::ws_client::create_client;

generate_state! {
    message_container_ref,
    username_ref,
    joinbtn_ref,
    textarea_ref,
    input_ref,
    sendbtn_ref,
}

thread_local! {
    pub static WS_CLIENT: EventClient = create_client();
    pub static USERNAME: Mutex<Option<String>> = Mutex::new(None);
    pub static ID: Mutex<Option<Uuid>> = Mutex::new(None);
    pub static WS_STATE: Mutex<PayloadMeta> = Mutex::new(PayloadMeta::new())
}

pub fn get_username() -> Option<String> {
    USERNAME.with(|inner| {
        inner
            .lock()
            .ok()
            .and_then(|mut opt| Some(opt.deref_mut().clone()))
            .flatten()
    })
}

pub fn set_username(name: String) {
    USERNAME.with(|inner| {
        inner.lock().ok().map(|mut mutguard_opt| {
            let opt = mutguard_opt.deref_mut();
            *opt = Some(name);
        });
    });
}

pub fn get_session_id() -> Option<Uuid> {
    ID.with(|inner| {
        inner
            .lock()
            .ok()
            .and_then(|mut opt| Some(opt.deref_mut().clone()))
            .flatten()
    })
}

pub fn set_session_id(name: Uuid) {
    ID.with(|inner| {
        inner.lock().ok().map(|mut mutguard_opt| {
            let opt = mutguard_opt.deref_mut();
            *opt = Some(name);
        });
    });
}

pub fn get_ws_state() -> Option<PayloadMeta> {
    WS_STATE.with(|inner| {
        inner
            .lock()
            .ok()
            .and_then(|mut opt| Some(opt.deref_mut().clone()))
    })
}

pub fn set_ws_state(meta: PayloadMeta) {
    WS_STATE.with(|inner| {
        inner.lock().ok().map(|mut mutguard_opt| {
            let opt = mutguard_opt.deref_mut();
            *opt = meta;
        });
    });
}
