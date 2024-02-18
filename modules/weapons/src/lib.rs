#[cfg_attr(feature = "load_configuration", deprecated(note = "This feature will be progressively changed to another implementation"))]

use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod missiles;
pub mod shells;
pub mod torpedo;
pub mod bullets;
pub mod firearm;

/// Speed in meters per second
///
/// If the speed is negative, the damage will be instantly applied
pub type Speed = f32;

pub type WeaponID = Uuid;

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
