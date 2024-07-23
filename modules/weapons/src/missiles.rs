//! This module define missiles

use crate::{Damages, Speed, WeaponInformations};
use serde::{Deserialize, Serialize};

/// The projectile type is the type of trajectory the missile will be using
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[repr(u8)]
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
    Ballistic = 1,
}

impl TryFrom<i64> for ProjectileType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ProjectileType::Cruise),
            1 => Ok(ProjectileType::Ballistic),
            _ => Err(()),
        }
    }
}

/// The missile guidance type is the type of guidance that is used in the missile
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[repr(u8)]
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
    Radio = 4,
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
            _ => Err(()),
        }
    }
}

/// The warhead type is the type of warhead that is used in the missile
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[repr(u8)]
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
    Emp = 7,
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
            _ => Err(()),
        }
    }
}

/// The warhead charge is the type of explosive charge that is used in the warhead
#[derive(Clone, Default, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
#[repr(u8)]
pub enum WarheadCharge {
    /// A standard explosive charge
    #[default]
    Standard = 0,
    /// A chemical charge, will release a cloud of toxic gas
    Chemical = 1,
    /// A nuclear charge, no further explanation needed
    Nuclear = 2,
    /// A biological charge, will release a cloud with a deadly virus or bacteria
    Biological = 3,
}

impl TryFrom<i64> for WarheadCharge {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(WarheadCharge::Standard),
            1 => Ok(WarheadCharge::Chemical),
            2 => Ok(WarheadCharge::Nuclear),
            3 => Ok(WarheadCharge::Biological),
            _ => Err(()),
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
#[derive(Clone, Debug, Serialize, Deserialize)]
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

    /// The information about the missile
    informations: WeaponInformations,
    damages: Damages,
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// ```
    pub fn new(guidance: MissileGuidanceType, projectile: ProjectileType) -> Self {
        Self {
            guidance,
            projectile,
            hypersonic: false,
            warhead: WarheadType::Cruise,
            warhead_charge: WarheadCharge::Standard,
            warhead_count: 1,
            informations: WeaponInformations::default(),
            damages: Damages::default(),
        }
    }

    /// Get the type of missile guidance
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{DEFAULT_SPEED, Missile, MissileGuidanceType, ProjectileType};
    ///
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_speed(), DEFAULT_SPEED);
    /// ```
    pub fn get_speed(&self) -> Speed {
        self.informations.speed
    }

    /// Set the speed of the missile
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::missiles::{DEFAULT_SPEED, Missile, MissileGuidanceType, ProjectileType};
    ///
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_speed(), DEFAULT_SPEED);
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType, WarheadType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType, WarheadType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType, WarheadCharge};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType, WarheadCharge};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_warhead_count(), 1);
    /// missile.set_warhead_count(2);
    /// assert_eq!(missile.get_warhead_count(), 2);
    /// ```
    pub fn set_warhead_count(&mut self, warhead_count: WarheadCount) {
        self.warhead_count = warhead_count;
    }

    /// Get the information about the missile
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    ///
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
    /// ```
    /// use weapons::missiles::{Missile, MissileGuidanceType, ProjectileType};
    /// use weapons::WeaponInformations;
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
    /// assert_eq!(missile.get_informations().name, String::new());
    /// missile.set_informations(WeaponInformations {
    ///   name: "Exocet".to_string(),
    ///   caliber: 0.0,
    ///   speed: 315.0,
    ///   range: 180.0,
    ///   country_reference: "fr".to_string()
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
    fn test_informations() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruise);
        assert_eq!(missile.get_informations().name, String::new());
        missile.set_informations(WeaponInformations {
            name: "Exocet".to_string(),
            caliber: 0.0,
            speed: 315.0,
            range: 180.0,
            country_reference: "FR".to_string(),
        });
        assert_eq!(missile.get_informations().name, "Exocet".to_string());
        assert_eq!(missile.get_informations().caliber, 0.0);
        assert_eq!(missile.get_informations().speed, 315.0);
        assert_eq!(missile.get_informations().range, 180.0);
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
            satellite: 11.0,
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
