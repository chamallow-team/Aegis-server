use bevy_ecs::prelude::Schedule;
use bevy_ecs::schedule::ScheduleLabel;

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NetMessageReceiver;

impl NetMessageReceiver {
    pub(crate) fn create() -> Schedule {
        Schedule::new(Self)
    }
}

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct NetMessageSender;

impl NetMessageSender {
    pub(crate) fn create() -> Schedule {
        Schedule::new(Self)
    }
}
