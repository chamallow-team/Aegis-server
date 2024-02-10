use serde::{Deserialize, Serialize};

pub mod missiles;
pub mod shells;
pub mod torpedo;

/// Speed in meters per second
///
/// If the speed is negative, the damage will be instantly applied
pub type Speed = f32;

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

#[cfg(feature = "load_configuration")]
pub mod loader {
    use std::{fs, io};
    use std::collections::HashMap;
    use std::path::PathBuf;
    use toml::Value;
    use crate::{Damages, WeaponInformations};
    use crate::missiles::Missile;
    use crate::shells::Shell;

    pub(crate) fn parse_weapons_information(t: &Value) -> Result<WeaponInformations, toml::de::Error> {
        t.clone().try_into()
    }

    pub(crate) fn parse_damages(t: &Value) -> Result<Damages, toml::de::Error> {
        t.clone().try_into()
    }

    pub type WeaponID = String;

    /// Store all the weapons
    ///
    /// This structure is used to store all the weapons that are loaded from the files in the weapons folder
    #[derive(Default, Clone, Debug)]
    pub struct WeaponsStore {
        pub missiles: HashMap<WeaponID, Missile>,
        pub shell: HashMap<WeaponID, Shell>
    }

    pub fn load<T: Into<PathBuf>>(path: T) -> io::Result<WeaponsStore> {
        let folder = path.into();

        let mut store = WeaponsStore::default();
        let files = fs::read_dir(folder)?
            .filter_map(|d| d.ok());

        for dir in files {
            let file_metadata = dir.metadata()?;

            // get the name of the file in lowercase, send a warning (with println) if getting the name fails, and continue
            let file_name = match dir.file_name().into_string() {
                Ok(name) => name.to_lowercase(),
                Err(_) => {
                    // TODO use a proper logging method
                    println!("\x1b[33mWarning: cannot get the name of the file {:?}\x1b[0m", dir.path());
                    continue;
                }
            };

            if file_metadata.is_file() {
                match file_name.as_str() {
                    "shells.toml" => crate::shells::loader::read_shells(dir.path(), &mut store)?,
                    "missiles.toml" => crate::missiles::loader::read_missiles(dir.path(), &mut store)?,
                    // TODO use a proper logging method
                    _ => println!("\x1b[33mWarning: the file {:?} is unknown\x1b[0m", dir.path())
                }
            }
        }

        Ok(store)
    }
}