use bevy_ecs::prelude::Schedule;
use bevy_ecs::schedule::ScheduleLabel;

/// All tasks that need to be run at the start of the server
#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct StartSchedule;

impl StartSchedule {
    pub(crate) fn create() -> Schedule {
        let mut schedule = Schedule::new(Self);
        schedule.add_systems(super::on_start);

        schedule
    }
}

#[derive(ScheduleLabel, Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) struct UpdateSchedule;

impl UpdateSchedule {
    pub(crate) fn create() -> Schedule {
        let mut schedule = Schedule::new(Self);
        schedule.add_systems((super::query_ships, super::hello));

        schedule
    }
}
