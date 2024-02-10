/// Enumeration representing different types of propulsion for torpedoes.
pub enum PropulsionType {
    /// Standard propulsion method.
    Standard,
    /// Stealthy propulsion for silent operations.
    Sneaky,
    /// Extremely quiet propulsion for covert missions.
    FuckingSilent,
}

/// Enumeration representing different types of guidance systems for torpedoes.
pub enum GuidanceType {
    /// No sonar, follows a linear trajectory.
    Simple,
    /// Activates after a specified duration, initiates a search pattern for the target, and aims at any target pinged by sonar.
    Sonar,
    /// Similar to Sonar, but with a guiding cable.
    Guided,
    /// Propelled by a missile initially, then launched as a standard torpedo.
    AirSea,
}
