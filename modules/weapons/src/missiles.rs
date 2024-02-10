use serde::{Deserialize, Serialize};
use crate::{Damages, Speed, WeaponInformations};

/// The projectile type is the type of trajectory the missile will be using
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ProjectileType {   
    /// The missile is guided by a human operator
    ///
    /// The missile is able to change his trajectory after launch and is able to follow a target.
    /// He uses a cruise trajectory with a low altitude.
    Cruise = 0,
    /// The missile is guided by a radar
    ///
    /// His trajectory is a parabola, and he can go up to 100 km in altitude.
    /// He can't change his trajectory after launch.
    Ballistic = 1
}

impl TryFrom<i64> for ProjectileType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProjectileType::Cruise),
            1 => Ok(ProjectileType::Ballistic),
            _ => Err(())
        }
    }
}

/// The missile guidance type is the type of guidance that is used in the missile
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum MissileGuidanceType {
    /// The missile is guided by a human operator
    Laser = 0,
    /// The missile is guided by a radar
    Radar = 1,
    /// The missile is guided by a heat source
    Heat = 2,
    /// The missile is guided by a GPS signal
    Gps = 3,
    /// The missile is guided by a radio signal
    Radio = 4
}

impl TryFrom<i64> for MissileGuidanceType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(MissileGuidanceType::Laser),
            1 => Ok(MissileGuidanceType::Radar),
            2 => Ok(MissileGuidanceType::Heat),
            3 => Ok(MissileGuidanceType::Gps),
            4 => Ok(MissileGuidanceType::Radio),
            _ => Err(())
        }
    }
}

/// The warhead type is the type of warhead that is used in the missile
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum WarheadType {
    /// Cruise missile
    Cruise = 0,
    /// Anti-ship missile
    AntiShip = 1,
    /// Anti-aircraft missile
    AntiAircraft = 2,
    /// Anti-ballistic missile
    Abm = 3,
    /// Short-range ballistic missile
    Srbm = 4,
    /// Medium-range ballistic missile
    Mrbm = 5,
    /// Intercontinental ballistic missile
    Icbm = 6,
    /// Electromagnetic pulse missile
    ///
    /// # Note
    /// Should only be used with a nuclear charge, otherwise it's useless and will do nothing
    Emp = 7
}

impl TryFrom<i64> for WarheadType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WarheadType::Cruise),
            1 => Ok(WarheadType::AntiShip),
            2 => Ok(WarheadType::AntiAircraft),
            3 => Ok(WarheadType::Abm),
            4 => Ok(WarheadType::Srbm),
            5 => Ok(WarheadType::Mrbm),
            6 => Ok(WarheadType::Icbm),
            7 => Ok(WarheadType::Emp),
            _ => Err(())
        }
    }
}

/// The warhead charge is the type of explosive charge that is used in the warhead
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum WarheadCharge {
    /// A standard explosive charge
    #[default]
    Standard = 0,
    /// A chemical charge, will release a cloud of toxic gas
    Chemical = 1,
    /// A nuclear charge, no further explanation needed
    Nuclear = 2,
    /// A biological charge, will release a cloud with a deadly virus or bacteria
    Biological = 3
}

impl TryFrom<i64> for WarheadCharge {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WarheadCharge::Standard),
            1 => Ok(WarheadCharge::Chemical),
            2 => Ok(WarheadCharge::Nuclear),
            3 => Ok(WarheadCharge::Biological),
            _ => Err(())
        }
    }
}

/// The warhead count is the number of warhead that is used in the missile
pub type WarheadCount = u32;

/// Represents a missile
///
/// This instance can be used in two ways:
/// - Represent a missile that is fired by a unit
/// - Represent a missile for its information, such as in the research tree
#[derive(Clone, Debug)]
pub struct Missile {
    /// The guidance type of the missile
    guidance: MissileGuidanceType,
    /// The type of projectile that is used in the missile
    projectile: ProjectileType,
    /// If the missile is hypersonic, it means that he is able to go faster than Mach 5 and can
    /// dodge anti-missile systems more easily
    hypersonic: bool,

    /// The type of warhead that is used in the missile
    warhead: WarheadType,
    /// The type of explosive charge that is used in the warhead
    warhead_charge: WarheadCharge,
    /// The number of warhead that is used in the missile
    warhead_count: WarheadCount,

    /// The position of the missile
    position: MissileCoordinate,

    /// The information about the missile
    informations: WeaponInformations,
    damages: Damages
}

/// Default speed of a missile in meters per second
pub const DEFAULT_SPEED: Speed = 0.0;

impl Missile {
    /// Create a new missile
    ///
    /// # Arguments
    ///
    /// * `guidance` - The guidance type of the missile
    /// * `projectile` - The type of projectile that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// ```
    pub fn new(guidance: MissileGuidanceType, projectile: ProjectileType) -> Self {
        Self {
            guidance, projectile,
            hypersonic: false,
            warhead: WarheadType::Cruise,
            warhead_charge: WarheadCharge::Standard,
            warhead_count: 1,
            position: MissileCoordinate {
                from: (0.0, 0.0),
                to: (0.0, 0.0),
                progress: 0.0
            },
            informations: WeaponInformations::default(),
            damages: Damages::default()
        }
    }

    /// Get the type of missile guidance
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
    /// ```
    pub fn get_missile_type(&self) -> MissileGuidanceType {
        self.guidance
    }

    /// Set the type of missile guidance
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
    /// missile.set_missile_type(MissileGuidanceType::Heat);
    /// assert_eq!(missile.get_missile_type(), MissileGuidanceType::Heat);
    /// ```
    pub fn set_missile_type(&mut self, guidance: MissileGuidanceType) {
        self.guidance = guidance;
    }

    /// Get the type of projectile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_projectile_type(), ProjectileType::Cruise);
    /// ```
    pub fn get_projectile_type(&self) -> ProjectileType {
        self.projectile
    }

    /// Set the type of projectile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_projectile_type(), ProjectileType::Cruise);
    /// missile.set_projectile_type(ProjectileType::Ballistic);
    /// assert_eq!(missile.get_projectile_type(), ProjectileType::Ballistic);
    /// ```
    pub fn set_projectile_type(&mut self, projectile: ProjectileType) {
        self.projectile = projectile;
    }

    /// Get the speed of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_speed(), 1000.0);
    /// ```
    pub fn get_speed(&self) -> Speed {
        self.informations.speed
    }

    /// Set the speed of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_speed(), 1000.0);
    /// missile.set_speed(2000.0);
    /// assert_eq!(missile.get_speed(), 2000.0);
    /// ```
    pub fn set_speed(&mut self, speed: Speed) {
        self.informations.speed = speed;
    }

    /// Return a boolean that indicates if the missile is hypersonic
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert!(!missile.is_hypersonic());
    /// ```
    pub fn is_hypersonic(&self) -> bool {
        self.hypersonic
    }

    /// Set if whether the missile is hypersonic or not
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert!(!missile.is_hypersonic());
    /// missile.set_hypersonic(true);
    /// assert!(missile.is_hypersonic());
    /// ```
    pub fn set_hypersonic(&mut self, hypersonic: bool) {
        self.hypersonic = hypersonic;
    }

    /// Get the type of warhead that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_type(), WarheadType::Cruise);
    /// ```
    pub fn get_warhead_type(&self) -> WarheadType {
        self.warhead
    }

    /// Set the type of warhead that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_type(), WarheadType::Cruise);
    /// missile.set_warhead_type(WarheadType::AntiShip);
    /// assert_eq!(missile.get_warhead_type(), WarheadType::AntiShip);
    /// ```
    pub fn set_warhead_type(&mut self, warhead: WarheadType) {
        self.warhead = warhead;
    }

    /// Get the type of explosive charge that is used in the warhead
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
    /// ```
    pub fn get_warhead_charge(&self) -> WarheadCharge {
        self.warhead_charge
    }

    /// Set the type of explosive charge that is used in the warhead
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
    /// missile.set_warhead_charge(WarheadCharge::Chemical);
    /// assert_eq!(missile.get_warhead_charge(), WarheadCharge::Chemical);
    /// ```
    pub fn set_warhead_charge(&mut self, warhead_charge: WarheadCharge) {
        self.warhead_charge = warhead_charge;
    }

    /// Get the number of warhead that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_count(), 1);
    /// ```
    pub fn get_warhead_count(&self) -> WarheadCount {
        self.warhead_count
    }

    /// Set the number of warhead that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_count(), 1);
    /// missile.set_warhead_count(2);
    /// assert_eq!(missile.get_warhead_count(), 2);
    /// ```
    pub fn set_warhead_count(&mut self, warhead_count: WarheadCount) {
        self.warhead_count = warhead_count;
    }

    /// Get the position of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_position().from, (0.0, 0.0));
    /// ```
    pub fn get_position(&self) -> &MissileCoordinate {
        &self.position
    }

    /// Get the mutable position of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_position().from, (0.0, 0.0));
    /// missile.get_position_mut().from = (1.0, 1.0);
    pub fn get_position_mut(&mut self) -> &mut MissileCoordinate {
        &mut self.position
    }

    /// Set the position of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_position().from, (0.0, 0.0));
    /// missile.set_position(MissileCoordinate {
    ///    from: (1.0, 1.0),
    ///    to: (2.0, 2.0),
    ///    progress: 0.5
    /// });
    /// assert_eq!(missile.get_position().from, (1.0, 1.0));
    /// assert_eq!(missile.get_position().to, (2.0, 2.0));
    /// assert_eq!(missile.get_position().progress, 0.5);
    /// ```
    pub fn set_position(&mut self, position: MissileCoordinate) {
        self.position = position;
    }

    /// Get the information about the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_informations().name, String::new());
    /// ```
    pub fn get_informations(&self) -> &WeaponInformations {
        &self.informations
    }

    /// Get the mutable information about the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_informations().name, String::new());
    /// missile.get_informations_mut().name = "Exocet".to_string();
    /// assert_eq!(missile.get_informations().name, "Exocet".to_string());
    /// ```
    pub fn get_informations_mut(&mut self) -> &mut WeaponInformations {
        &mut self.informations
    }

    /// Set the information about the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_informations().name, String::new());
    /// missile.set_informations(WeaponInformations {
    ///    name: "Exocet".to_string(),
    ///   caliber: 0.0,
    ///   damages: Default::default(),
    ///   speed: 315.0,
    ///   range: 180.0,
    ///   fire_rate: 1.0
    /// });
    /// assert_eq!(missile.get_informations().name, "Exocet".to_string());
    /// ```
    pub fn set_informations(&mut self, informations: WeaponInformations) {
        self.informations = informations;
    }

    /// Get the damages of the missile
    pub fn get_damages(&self) -> &Damages {
        &self.damages
    }

    /// Get the mutable damages of the missile
    pub fn get_damages_mut(&mut self) -> &mut Damages {
        &mut self.damages
    }

    /// Set the damages of the missile
    pub fn set_damages(&mut self, damages: Damages) {
        self.damages = damages;
    }
}

#[derive(Clone, Debug)]
#[allow(unused)] // TODO remove this ugly hack
pub struct MissileCoordinate {
    from: (f32, f32),
    to: (f32, f32),
    progress: f32
}

#[cfg(test)]
mod test {
    #[test]
    fn test_default_missile() {
        use super::*;

        let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Cruise);
        assert_eq!(missile.get_speed(), DEFAULT_SPEED);
        assert!(!missile.is_hypersonic());
        assert_eq!(missile.get_warhead_type(), WarheadType::Cruise);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
        assert_eq!(missile.get_warhead_count(), 1);
        assert_eq!(missile.get_position().from, (0.0, 0.0));
    }

    #[test]
    fn test_missile_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
        missile.set_missile_type(MissileGuidanceType::Heat);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Heat);
    }

    #[test]
    fn test_projectile_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Cruise);
        missile.set_projectile_type(ProjectileType::Ballistic);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Ballistic);
    }

    #[test]
    fn test_speed() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_speed(), DEFAULT_SPEED);
        missile.set_speed(2000.0);
        assert_eq!(missile.get_speed(), 2000.0);
    }

    #[test]
    fn test_hypersonic() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert!(!missile.is_hypersonic());
        missile.set_hypersonic(true);
        assert!(missile.is_hypersonic());
    }

    #[test]
    fn test_warhead_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_warhead_type(), WarheadType::Cruise);
        missile.set_warhead_type(WarheadType::AntiShip);
        assert_eq!(missile.get_warhead_type(), WarheadType::AntiShip);
    }

    #[test]
    fn test_warhead_charge() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
        missile.set_warhead_charge(WarheadCharge::Chemical);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Chemical);
    }

    #[test]
    fn test_warhead_count() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_warhead_count(), 1);
        missile.set_warhead_count(2);
        assert_eq!(missile.get_warhead_count(), 2);
    }

    #[test]
    fn test_position() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_position().from, (0.0, 0.0));
        missile.set_position(MissileCoordinate {
            from: (1.0, 1.0),
            to: (2.0, 2.0),
            progress: 0.5
        });
        assert_eq!(missile.get_position().from, (1.0, 1.0));
        assert_eq!(missile.get_position().to, (2.0, 2.0));
        assert_eq!(missile.get_position().progress, 0.5);
    }

    #[test]
    fn test_informations() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_informations().name, String::new());
        missile.set_informations(WeaponInformations {
            name: "Exocet".to_string(),
            caliber: 0.0,
            speed: 315.0,
            range: 180.0,
            country_reference: "FR".to_string()
        });
        assert_eq!(missile.get_informations().name, "Exocet".to_string());
        assert_eq!(missile.get_informations().caliber, 0.0);
        assert_eq!(missile.get_informations().speed, 315.0);
        assert_eq!(missile.get_informations().range, 180.0);
    }

    #[test]
    fn test_missile_coordinate() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_position().from, (0.0, 0.0));
        missile.get_position_mut().from = (1.0, 1.0);
        assert_eq!(missile.get_position().from, (1.0, 1.0));
    }

    #[test]
    fn test_missile_coordinate_to() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_position().to, (0.0, 0.0));
        missile.get_position_mut().to = (1.0, 1.0);
        assert_eq!(missile.get_position().to, (1.0, 1.0));
    }

    #[test]
    fn test_missile_coordinate_progress() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_position().progress, 0.0);
        missile.get_position_mut().progress = 0.5;
        assert_eq!(missile.get_position().progress, 0.5);
    }

    #[test]
    fn test_damages() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_damages().building, 0.0);
        assert_eq!(missile.get_damages().infantry, 0.0);
        assert_eq!(missile.get_damages().vehicle, 0.0);
        assert_eq!(missile.get_damages().armored_vehicle, 0.0);
        assert_eq!(missile.get_damages().tank, 0.0);
        assert_eq!(missile.get_damages().helicopter, 0.0);
        assert_eq!(missile.get_damages().plane, 0.0);
        assert_eq!(missile.get_damages().ship, 0.0);
        assert_eq!(missile.get_damages().submarine, 0.0);
        assert_eq!(missile.get_damages().missile, 0.0);
        assert_eq!(missile.get_damages().satellite, 0.0);

        missile.set_damages(Damages {
            building: 1.0,
            infantry: 2.0,
            vehicle: 3.0,
            armored_vehicle: 4.0,
            tank: 5.0,
            helicopter: 6.0,
            plane: 7.0,
            ship: 8.0,
            submarine: 9.0,
            missile: 10.0,
            satellite: 11.0
        });

        assert_eq!(missile.get_damages().building, 1.0);
        assert_eq!(missile.get_damages().infantry, 2.0);
        assert_eq!(missile.get_damages().vehicle, 3.0);
        assert_eq!(missile.get_damages().armored_vehicle, 4.0);
        assert_eq!(missile.get_damages().tank, 5.0);
        assert_eq!(missile.get_damages().helicopter, 6.0);
        assert_eq!(missile.get_damages().plane, 7.0);
        assert_eq!(missile.get_damages().ship, 8.0);
        assert_eq!(missile.get_damages().submarine, 9.0);
        assert_eq!(missile.get_damages().missile, 10.0);
        assert_eq!(missile.get_damages().satellite, 11.0);
    }
}

#[cfg(feature = "load_configuration")]
pub(crate) mod loader {
    use std::{fs, io};
    use std::path::PathBuf;
    use toml::{Table, Value};
    use crate::loader::{parse_damages, parse_weapons_information, WeaponsStore};
    use crate::missiles::{Missile, MissileGuidanceType, ProjectileType, WarheadCharge, WarheadType};

    pub(crate) fn read_missiles(dir: PathBuf, store: &mut WeaponsStore) -> io::Result<()> {
        let content = fs::read_to_string(&dir)?;

        let table = match toml::from_str::<Table>(content.as_str()) {
            Ok(table) => table,
            Err(e) => {
                // TODO use a proper logging method
                println!("\x1b[33mWarning: cannot parse the toml content at {dir:?}: {e:?}\x1b[0m");
                return Ok(())
            }
        };

        for (k, v) in table.iter() {
            if !v.is_table() {
                // TODO use a proper logging method
                println!("\x1b[33mWarning: the value of the key {k:?} at {dir:?} is not a table\x1b[0m");
                continue;
            }

            if let Some(m) = parse_missile(&dir, v.as_table().unwrap()) {
                store.missiles.insert(k.into(), m);
            }
        }

        Ok(())
    }

    const UNWANTED_KEYS: &[&str] = &["guidance", "projectile", "hypersonic", "warhead", "warhead_charge", "warhead_count"];

    fn parse_missile(dir: &PathBuf, t: &Table) -> Option<Missile> {
        let guidance = MissileGuidanceType::try_from(
            get_type(t, "guidance", dir)?.as_integer()?
        ).ok()?;
        let projectile = ProjectileType::try_from(
            get_type(t, "projectile", dir)?.as_integer()?
        ).ok()?;
        let hypersonic = get_type(t, "hypersonic", dir)?.as_bool()?;
        let warhead = WarheadType::try_from(
            get_type(t, "warhead", dir)?.as_integer()?
        ).ok()?;
        let warhead_charge = WarheadCharge::try_from(
            get_type(t, "warhead_charge", dir)?.as_integer()?
        ).ok()?;
        let warhead_count = get_type(t, "warhead_count", dir)?.as_integer()? as u32;

        let mut missile = Missile::new(guidance, projectile);
        missile.set_hypersonic(hypersonic);
        missile.set_warhead_type(warhead);
        missile.set_warhead_charge(warhead_charge);
        missile.set_warhead_count(warhead_count);

        for (k, v) in t.iter().filter(|(k, _)| !UNWANTED_KEYS.contains(&k.to_lowercase().as_str())) {
            match k.as_str() {
                "informations" => match parse_weapons_information(v) {
                    Ok(t) => missile.set_informations(t),
                    Err(e) => {
                        // TODO use a proper logging method
                        println!("\x1b[33mWarning: cannot parse the informations of the missile at {dir:?}: {e:?}\x1b[0m");
                        return None;
                    }
                },
                "damages" => match parse_damages(v) {
                    Ok(d) => missile.set_damages(d),
                    Err(e) => {
                        // TODO use a proper logging method
                        println!("\x1b[33mWarning: cannot parse the damages of the missile at {dir:?}: {e:?}\x1b[0m");
                        return None;
                    }
                },
                _ => {
                    // TODO use a proper logging method
                    println!("\x1b[33mWarning: the key {k:?} of the missile at {dir:?} is unknown\x1b[0m");
                    continue;
                }
            }
        }

        Some(missile)
    }

    fn get_type<'a>(t: &'a Table, k: &str, dir: &PathBuf) -> Option<&'a Value> {
        if !t.contains_key(k) {
            // TODO use a proper logging method
            println!("\x1b[33mWarning: the missile at {dir:?} does not have a key for {k:?}\x1b[0m");
            return None;
        }
        Some(t.get(k).unwrap())
    }
}