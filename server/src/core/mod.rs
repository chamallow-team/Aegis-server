//! This module will contain all the game's mechanic: from the ECS system to the updates

use crate::core::net::{NetMessageReceiver, NetMessageSender};
use crate::core::systems::{StartSchedule, UpdateSchedule};
use bevy_ecs::prelude::*;
use std::time::{Duration, Instant};
use tokio::time::sleep;

mod net;
mod systems;

pub(crate) struct GameCore {
    world: World,
}

pub(crate) async fn create_core_game() -> GameCore {
    GameCore {
        world: World::new(),
    }
}

impl GameCore {
    pub(crate) async fn launch(mut self) {
        // Game logic
        self.world.add_schedule(StartSchedule::create());
        self.world.add_schedule(UpdateSchedule::create());

        // Net
        self.world.add_schedule(NetMessageReceiver::create());
        self.world.add_schedule(NetMessageSender::create());

        // Start the loop
        self.core_loop().await;
    }

    async fn core_loop(&mut self) {
        self.world.run_schedule(StartSchedule);

        let mut tick = 0;
        loop {
            tick += 1;

            println!("Tick n°{tick}");
            let n = Instant::now();
            // Process the user events
            self.world.run_schedule(NetMessageReceiver);

            // Update
            self.world.run_schedule(UpdateSchedule);

            // Send events to the users
            self.world.run_schedule(NetMessageSender);

            if n.elapsed().as_millis() < 1000 {
                // Less than 1 s passed, wait
                let elapsed: u128 = 1000 - n.elapsed().as_millis();
                sleep(Duration::from_millis(elapsed as u64)).await;
            }
        }
    }
}

#[derive(Component, Debug)]
pub struct NavyShip {
    #[allow(dead_code)]
    model: u16,
}

fn on_start(mut commands: Commands) {
    for _ in 0..2 {
        commands.spawn(NavyShip { model: 0 });
    }
}

fn query_ships(query: Query<'_, '_, &NavyShip>) {
    for ship in &query {
        println!("{:?}", ship);
    }
}

fn hello() {
    println!("hello");
}
