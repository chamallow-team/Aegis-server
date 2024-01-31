use serde::{Deserialize, Serialize};
use crate::{Speed, WeaponInformations};

/// The projectile type is the type of trajectory the missile will be using
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ProjectileType {   
    /// The missile is guided by a human operator
    ///
    /// The missile is able to change his trajectory after launch and is able to follow a target.
    /// He uses a cruise trajectory with a low altitude.
    Cruiser = 0,
    /// The missile is guided by a radar
    ///
    /// His trajectory is a parabola, and he can go up to 100 km in altitude.
    /// He can't change his trajectory after launch.
    Ballistic = 1
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

/// The warhead type is the type of warhead that is used in the missile
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum WarheadType {
    /// Cruise missile
    Cruiser = 0,
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

/// The warhead count is the number of warhead that is used in the missile
pub type WarheadCount = u32;

/// Represents a missile
///
/// This instance can be used in two ways:
/// - Represent a missile that is fired by a unit
/// - Represent a missile for its information, such as in the research tree
#[derive(Clone)]
pub struct Missile {
    /// The guidance type of the missile
    guidance: MissileGuidanceType,
    /// The type of projectile that is used in the missile
    projectile: ProjectileType,
    /// The speed of the missile
    speed: Speed,
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
    informations: WeaponInformations
}

/// Default speed of a missile in meters per second
const DEFAULT_SPEED: Speed = 1000.0;

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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// ```
    pub fn new(guidance: MissileGuidanceType, projectile: ProjectileType) -> Self {
        Self {
            guidance, projectile,
            speed: DEFAULT_SPEED,
            hypersonic: false,
            warhead: WarheadType::Cruiser,
            warhead_charge: WarheadCharge::Standard,
            warhead_count: 1,
            position: MissileCoordinate {
                from: (0.0, 0.0),
                to: (0.0, 0.0),
                progress: 0.0
            },
            informations: WeaponInformations::default()
        }
    }

    /// Get the type of missile guidance
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_projectile_type(), ProjectileType::Cruiser);
    /// ```
    pub fn get_projectile_type(&self) -> ProjectileType {
        self.projectile
    }

    /// Set the type of projectile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_projectile_type(), ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_speed(), 1000.0);
    /// ```
    pub fn get_speed(&self) -> Speed {
        self.speed
    }

    /// Set the speed of the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_speed(), 1000.0);
    /// missile.set_speed(2000.0);
    /// assert_eq!(missile.get_speed(), 2000.0);
    /// ```
    pub fn set_speed(&mut self, speed: Speed) {
        self.speed = speed;
    }

    /// Return a boolean that indicates if the missile is hypersonic
    ///
    /// # Example
    ///
    /// ```rs
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_warhead_type(), WarheadType::Cruiser);
    /// ```
    pub fn get_warhead_type(&self) -> WarheadType {
        self.warhead
    }

    /// Set the type of warhead that is used in the missile
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
    /// assert_eq!(missile.get_warhead_type(), WarheadType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
    /// let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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
}

#[derive(Clone)]
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

        let missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Cruiser);
        assert_eq!(missile.get_speed(), DEFAULT_SPEED);
        assert!(!missile.is_hypersonic());
        assert_eq!(missile.get_warhead_type(), WarheadType::Cruiser);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
        assert_eq!(missile.get_warhead_count(), 1);
        assert_eq!(missile.get_position().from, (0.0, 0.0));
    }

    #[test]
    fn test_missile_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Laser);
        missile.set_missile_type(MissileGuidanceType::Heat);
        assert_eq!(missile.get_missile_type(), MissileGuidanceType::Heat);
    }

    #[test]
    fn test_projectile_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Cruiser);
        missile.set_projectile_type(ProjectileType::Ballistic);
        assert_eq!(missile.get_projectile_type(), ProjectileType::Ballistic);
    }

    #[test]
    fn test_speed() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_speed(), DEFAULT_SPEED);
        missile.set_speed(2000.0);
        assert_eq!(missile.get_speed(), 2000.0);
    }

    #[test]
    fn test_hypersonic() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert!(!missile.is_hypersonic());
        missile.set_hypersonic(true);
        assert!(missile.is_hypersonic());
    }

    #[test]
    fn test_warhead_type() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_warhead_type(), WarheadType::Cruiser);
        missile.set_warhead_type(WarheadType::AntiShip);
        assert_eq!(missile.get_warhead_type(), WarheadType::AntiShip);
    }

    #[test]
    fn test_warhead_charge() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Standard);
        missile.set_warhead_charge(WarheadCharge::Chemical);
        assert_eq!(missile.get_warhead_charge(), WarheadCharge::Chemical);
    }

    #[test]
    fn test_warhead_count() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_warhead_count(), 1);
        missile.set_warhead_count(2);
        assert_eq!(missile.get_warhead_count(), 2);
    }

    #[test]
    fn test_position() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
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

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_informations().name, String::new());
        missile.set_informations(WeaponInformations {
            name: "Exocet".to_string(),
            caliber: 0.0,
            damages: Default::default(),
            speed: 315.0,
            range: 180.0,
            fire_rate: 1.0
        });
        assert_eq!(missile.get_informations().name, "Exocet".to_string());
        assert_eq!(missile.get_informations().caliber, 0.0);
        assert_eq!(missile.get_informations().damages, Default::default());
        assert_eq!(missile.get_informations().speed, 315.0);
        assert_eq!(missile.get_informations().range, 180.0);
        assert_eq!(missile.get_informations().fire_rate, 1.0);
    }

    #[test]
    fn test_missile_coordinate() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_position().from, (0.0, 0.0));
        missile.get_position_mut().from = (1.0, 1.0);
        assert_eq!(missile.get_position().from, (1.0, 1.0));
    }

    #[test]
    fn test_missile_coordinate_to() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_position().to, (0.0, 0.0));
        missile.get_position_mut().to = (1.0, 1.0);
        assert_eq!(missile.get_position().to, (1.0, 1.0));
    }

    #[test]
    fn test_missile_coordinate_progress() {
        use super::*;

        let mut missile = Missile::new(MissileGuidanceType::Laser, ProjectileType::Cruiser);
        assert_eq!(missile.get_position().progress, 0.0);
        missile.get_position_mut().progress = 0.5;
        assert_eq!(missile.get_position().progress, 0.5);
    }
}
