use serde::{Deserialize, Serialize};

mod missiles;
mod shells;

/// Speed in meters per second
pub type Speed = f32;

/// Define the damages that a weapon can do
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub struct Damages {
    /// The damages that the weapon can do to a building
    pub building: f32,
    /// The damages that the weapon can do to an infantry
    pub infantry: f32,
    /// The damages that the weapon can do to a vehicle
    pub vehicle: f32,
    /// The damages that the weapon can do to a tank
    pub armored_vehicle: f32,
    /// The damages that the weapon can do to a tank
    pub tank: f32,
    /// The damages that the weapon can do to a helicopter
    pub helicopter: f32,
    /// The damages that the weapon can do to a plane
    pub plane: f32,
    /// The damages that the weapon can do to a ship
    pub ship: f32,
    /// The damages that the weapon can do to a submarine
    pub submarine: f32,
    /// The damages that the weapon can do to a missile
    pub missile: f32,
    /// The damages that the weapon can do to a satellite
    pub satellite: f32,
}

/// Define the information that a weapon can have
///
/// This structure is used to define the characteristics of a weapon
///
/// # Example
///
/// ```rs
/// let weapon = WeaponInformations {
///   name: "M4A1".to_string(),
///   caliber: 5.56,
///   damages: Damages::default(),
///   speed: 900.0,
///   range: 500.0,
///   fire_rate: 800.0
/// };
/// ```
#[derive(Clone, Default, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct WeaponInformations {
    pub name: String,
    pub caliber: f32,
    pub damages: Damages,
    /// The speed of the projectile in meters per second
    pub speed: Speed,
    pub range: f32,
    pub fire_rate: f32
}