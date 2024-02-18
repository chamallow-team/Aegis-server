use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use crate::bullets::Bullet;
use crate::firearm::FireArm;
use crate::missiles::Missile;
use crate::shells::Shell;
use crate::torpedo::Torpedo;

pub mod missiles;
pub mod shells;
pub mod torpedo;
pub mod bullets;
pub mod firearm;

/// Speed in meters per second
///
/// If the speed is negative, the damage will be instantly applied
pub type Speed = f32;

pub type WeaponID = String;

/// Contains every weapon
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct WeaponStore {
    missiles: HashMap<WeaponID, Missile>,
    torpedoes: HashMap<WeaponID, Torpedo>,
    shells: HashMap<WeaponID, Shell>,
    firearm: HashMap<WeaponID, FireArm>,
    bullets: HashMap<WeaponID, Bullet>
}

impl WeaponStore {
    /// Get all missiles
    pub fn get_missiles(&self) -> &HashMap<WeaponID, Missile> {
        &self.missiles
    }

    /// Get all missiles with a mutable reference
    pub fn get_missiles_mut(&mut self) -> &mut HashMap<WeaponID, Missile> {
        &mut self.missiles
    }

    /// Get a missile by its id
    pub fn get_missile(&self, id: impl Into<WeaponID>) -> Option<&Missile> {
        self.missiles.get(&id.into())
    }

    /// Get a missile by its id with a mutable reference
    pub fn get_missile_mut(&mut self, id: impl Into<WeaponID>) -> Option<&mut Missile> {
        self.missiles.get_mut(&id.into())
    }

    /// Add a missile to the store
    pub fn add_missile(&mut self, id: impl Into<WeaponID>, missile: Missile) {
        self.missiles.insert(id.into(), missile);
    }

    /// Remove a missile from the store
    pub fn remove_missile(&mut self, id: impl Into<WeaponID>) {
        self.missiles.remove(&id.into());
    }

    /// Get all torpedoes
    pub fn get_torpedoes(&self) -> &HashMap<WeaponID, Torpedo> {
        &self.torpedoes
    }

    /// Get all torpedoes with a mutable reference
    pub fn get_torpedoes_mut(&mut self) -> &mut HashMap<WeaponID, Torpedo> {
        &mut self.torpedoes
    }

    /// Get a torpedo by its id
    pub fn get_torpedo(&self, id: impl Into<WeaponID>) -> Option<&Torpedo> {
        self.torpedoes.get(&id.into())
    }

    /// Get a torpedo by its id with a mutable reference
    pub fn get_torpedo_mut(&mut self, id: impl Into<WeaponID>) -> Option<&mut Torpedo> {
        self.torpedoes.get_mut(&id.into())
    }

    /// Add a torpedo to the store
    pub fn add_torpedo(&mut self, id: impl Into<WeaponID>, torpedo: Torpedo) {
        self.torpedoes.insert(id.into(), torpedo);
    }

    /// Remove a torpedo from the store
    pub fn remove_torpedo(&mut self, id: impl Into<WeaponID>) {
        self.torpedoes.remove(&id.into());
    }

    /// Get all shells
    pub fn get_shells(&self) -> &HashMap<WeaponID, Shell> {
        &self.shells
    }

    /// Get all shells with a mutable reference
    pub fn get_shells_mut(&mut self) -> &mut HashMap<WeaponID, Shell> {
        &mut self.shells
    }

    /// Get a shell by its id
    pub fn get_shell(&self, id: impl Into<WeaponID>) -> Option<&Shell> {
        self.shells.get(&id.into())
    }

    /// Get a shell by its id with a mutable reference
    pub fn get_shell_mut(&mut self, id: impl Into<WeaponID>) -> Option<&mut Shell> {
        self.shells.get_mut(&id.into())
    }

    /// Add a shell to the store
    pub fn add_shell(&mut self, id: impl Into<WeaponID>, shell: Shell) {
        self.shells.insert(id.into(), shell);
    }

    /// Remove a shell from the store
    pub fn remove_shell(&mut self, id: impl Into<WeaponID>) {
        self.shells.remove(&id.into());
    }

    /// Get all firearms
    pub fn get_firearms(&self) -> &HashMap<WeaponID, FireArm> {
        &self.firearm
    }

    /// Get all firearms with a mutable reference
    pub fn get_firearms_mut(&mut self) -> &mut HashMap<WeaponID, FireArm> {
        &mut self.firearm
    }

    /// Get a firearm by its id
    pub fn get_firearm(&self, id: impl Into<WeaponID>) -> Option<&FireArm> {
        self.firearm.get(&id.into())
    }

    /// Get a firearm by its id with a mutable reference
    pub fn get_firearm_mut(&mut self, id: impl Into<WeaponID>) -> Option<&mut FireArm> {
        self.firearm.get_mut(&id.into())
    }

    /// Add a firearm to the store
    pub fn add_firearm(&mut self, id: impl Into<WeaponID>, firearm: FireArm) {
        self.firearm.insert(id.into(), firearm);
    }

    /// Remove a firearm from the store
    pub fn remove_firearm(&mut self, id: impl Into<WeaponID>) {
        self.firearm.remove(&id.into());
    }

    /// Get all bullets
    pub fn get_bullets(&self) -> &HashMap<WeaponID, Bullet> {
        &self.bullets
    }

    /// Get all bullets with a mutable reference
    pub fn get_bullets_mut(&mut self) -> &mut HashMap<WeaponID, Bullet> {
        &mut self.bullets
    }

    /// Get a bullet by its id
    pub fn get_bullet(&self, id: impl Into<WeaponID>) -> Option<&Bullet> {
        self.bullets.get(&id.into())
    }

    /// Get a bullet by its id with a mutable reference
    pub fn get_bullet_mut(&mut self, id: impl Into<WeaponID>) -> Option<&mut Bullet> {
        self.bullets.get_mut(&id.into())
    }

    /// Add a bullet to the store
    pub fn add_bullet(&mut self, id: impl Into<WeaponID>, bullet: Bullet) {
        self.bullets.insert(id.into(), bullet);
    }

    /// Remove a bullet from the store
    pub fn remove_bullet(&mut self, id: impl Into<WeaponID>) {
        self.bullets.remove(&id.into());
    }
}

/// Define the damages that a weapon can do
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub struct Damages {
    /// The damages that the weapon can do to a building
    #[serde(default)]
    pub building: f32,
    /// The damages that the weapon can do to an infantry
    #[serde(default)]
    pub infantry: f32,
    /// The damages that the weapon can do to a vehicle
    #[serde(default)]
    pub vehicle: f32,
    /// The damages that the weapon can do to a tank
    #[serde(default)]
    pub armored_vehicle: f32,
    /// The damages that the weapon can do to a tank
    #[serde(default)]
    pub tank: f32,
    /// The damages that the weapon can do to a helicopter
    #[serde(default)]
    pub helicopter: f32,
    /// The damages that the weapon can do to a plane
    #[serde(default)]
    pub plane: f32,
    /// The damages that the weapon can do to a ship
    #[serde(default)]
    pub ship: f32,
    /// The damages that the weapon can do to a submarine
    #[serde(default)]
    pub submarine: f32,
    /// The damages that the weapon can do to a missile
    #[serde(default)]
    pub missile: f32,
    /// The damages that the weapon can do to a satellite
    #[serde(default)]
    pub satellite: f32,
}

/// Define the information that a weapon can have
///
/// This structure is used to define the characteristics of a weapon
///
/// # Example
///
/// ```
/// use weapons::{Damages, WeaponInformations};
///
/// let weapon = WeaponInformations {
///   name: "M4A1".to_string(),
///   caliber: 5.56,
///   speed: 900.0,
///   range: 500.0,
///   country_reference: "fr".into()
/// };
/// ```
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct WeaponInformations {
    pub name: String,
    /// The caliber of the weapon in millimeters
    #[serde(default)]
    pub caliber: f32,
    /// The speed of the projectile in meters per second
    #[serde(default)]
    pub speed: Speed,
    /// The range in kilometers
    #[serde(default)]
    pub range: f32,
    /// The country reference of the weapon, it's used to know which country can use the weapon
    ///
    /// TODO Use a custom type instead of a String
    pub country_reference: String
}
