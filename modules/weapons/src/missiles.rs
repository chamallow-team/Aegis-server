use crate::Speed;

/// The projectile type is the type of projectile that is used in the missile
#[derive(Clone)]
pub enum ProjectileType {
    /// The missile is guided by a human operator
    Laser,
    /// The missile is guided by a radar
    Radar,
    /// The missile is guided by a heat source
    Heat,
    /// The missile is guided by a GPS signal
    Gps,
    /// The missile is guided by a radio signal
    Radio
}

/// The missile guidance type is the type of guidance that is used in the missile
#[derive(Clone)]
pub enum MissileGuidanceType {
    /// The missile is guided by a human operator
    ///
    /// The missile is able to change his trajectory after launch and is able to follow a target.
    /// He uses a cruise trajectory with a low altitude.
    Cruiser,
    /// The missile is guided by a radar
    ///
    /// His trajectory is a parabola, and he can go up to 100 km in altitude.
    /// He can't change his trajectory after launch.
    Ballistic
}

/// The warhead type is the type of warhead that is used in the missile
#[derive(Clone)]
pub enum WarheadType {
    /// Cruise missile
    Cruiser,
    /// Anti-ship missile
    AntiShip,
    /// Anti-aircraft missile
    AntiAircraft,
    /// Anti-ballistic missile
    Abm,
    /// Short-range ballistic missile
    Srbm,
    /// Medium-range ballistic missile
    Mrbm,
    /// Intercontinental ballistic missile
    Icbm,
    /// Electromagnetic pulse missile
    ///
    /// # Note
    /// Should only be used with a nuclear charge, otherwise it's useless and will do nothing
    Emp
}

/// The warhead charge is the type of explosive charge that is used in the warhead
#[derive(Clone, Default)]
pub enum WarheadCharge {
    /// A standard explosive charge
    #[default]
    Standard,
    /// A chemical charge, will release a cloud of toxic gas
    Chemical,
    /// A nuclear charge, no further explanation needed
    Nuclear,
    /// A biological charge, will release a cloud with a deadly virus or bacteria
    Biological
}

/// The warhead count is the number of warhead that is used in the missile
pub type WarheadCount = u32;

/// Represents a missile
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
    position: MissileCoordinate
}

#[derive(Clone)]
pub struct MissileCoordinate {
    from: (f32, f32),
    to: (f32, f32),
    progress: f32
}