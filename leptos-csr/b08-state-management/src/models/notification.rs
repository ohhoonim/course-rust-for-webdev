use gloo_timers::callback::Timeout;
use leptos::prelude::*;

#[derive(Clone)]
pub struct Notification {
    pub id: u32,
    pub message: String,
    pub level: Level,
}

#[derive(Clone, Copy)]
pub enum Level {
    Info,
    Success,
    Warning,
}

#[derive(Clone, Copy)]
pub struct NextNotificationId(pub RwSignal<u32>);

pub fn use_notify() -> impl Fn(String, Level) + Clone {
    let notifications = use_context::<RwSignal<Vec<Notification>>>().expect("notification context");
    let next_id = use_context::<NextNotificationId>().expect("notification id counter").0;
    move |message, level| {
        let id = next_id.get_untracked();
        next_id.set(id + 1);
        notifications.update(|items| items.push(Notification { id, message, level }));
        let handle = Timeout::new(3_000, move || {
            notifications.update(|items| items.retain(|item| item.id != id));
        });
        handle.forget();
    }
}