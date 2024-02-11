//! This crate is used to define torpedoes

use serde::{Deserialize, Serialize};
use crate::{Damages, WeaponInformations};

/// Enumeration representing different types of propulsion for torpedoes.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Serialize, Deserialize)]
pub enum PropulsionType {
    /// Standard propulsion method.
    Standard = 0,
    /// Stealthy propulsion for silent operations.
    Sneaky = 1,
    /// Extremely quiet propulsion for covert missions.
    FuckingSilent = 2,
}

impl TryFrom<i64> for PropulsionType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Standard),
            1 => Ok(Self::Sneaky),
            2 => Ok(Self::FuckingSilent),
            _ => Err(())
        }
    }
}


/// Enumeration representing different types of guidance systems for torpedoes.
#[derive(Clone, Debug, PartialEq, PartialOrd, Copy, Serialize, Deserialize)]
pub enum GuidanceType {
    /// No sonar, follows a linear trajectory.
    Simple = 0,
    /// Activates after a specified duration, initiates a search pattern for the target, and aims at any target pinged by sonar.
    Sonar = 1,
    /// Similar to Sonar, but with a guiding cable.
    Guided = 2,
    /// Propelled by a missile initially, then launched as a standard torpedo.
    AirSea = 3,
}

impl TryFrom<i64> for GuidanceType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Simple),
            1 => Ok(Self::Sonar),
            2 => Ok(Self::Guided),
            3 => Ok(Self::AirSea),
            _ => Err(())
        }
    }
}

/// A torpedo object
#[derive(Debug, Clone, PartialEq)]
pub struct Torpedo {
    guidance: GuidanceType,
    propulsion: PropulsionType,

    informations: WeaponInformations,
    damages: Damages
}

impl Torpedo {
    /// Create a new torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    ///
    /// let torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_guidance(), GuidanceType::Guided);
    /// assert_eq!(torpedo.get_propulsion(), PropulsionType::FuckingSilent);
    /// ```
    pub fn new(guidance: GuidanceType, propulsion: PropulsionType) -> Self {
        Self {
            guidance, propulsion,

            informations: WeaponInformations::default(),
            damages: Damages::default()
        }
    }

    /// Get the guidance of the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    ///
    /// let torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_guidance(), GuidanceType::Guided);
    /// ```
    pub fn get_guidance(&self) -> GuidanceType {
        self.guidance
    }

    /// Define a new guidance for the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    ///
    /// let mut torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_guidance(), GuidanceType::Guided);
    /// torpedo.set_guidance(GuidanceType::Sonar);
    /// assert_eq!(torpedo.get_guidance(), GuidanceType::Sonar);
    /// ```
    pub fn set_guidance(&mut self, guidance: GuidanceType) {
        self.guidance = guidance;
    }

    /// Get the propulsion type of the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    ///
    /// let torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_propulsion(), PropulsionType::FuckingSilent);
    /// ```
    pub fn get_propulsion(&self) -> PropulsionType {
        self.propulsion
    }

    /// Define a new propulsion type for this torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    ///
    /// let mut torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_propulsion(), PropulsionType::FuckingSilent);
    /// torpedo.set_propulsion(PropulsionType::Sneaky);
    /// assert_eq!(torpedo.get_propulsion(), PropulsionType::Sneaky);
    /// ```
    pub fn set_propulsion(&mut self, propulsion: PropulsionType) {
        self.propulsion = propulsion;
    }

    /// Get the information on the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    /// use weapons::WeaponInformations;
    ///
    /// let torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_informations(), &WeaponInformations::default());
    /// ```
    pub fn get_informations(&self) -> &WeaponInformations {
        &self.informations
    }

    /// Get the information on the torpedo with a mutable reference
    ///
    /// See Self::get_informations
    pub fn get_informations_mut(&mut self) -> &mut WeaponInformations {
        &mut self.informations
    }

    /// Get the damages given by the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::torpedo::{GuidanceType, PropulsionType, Torpedo};
    /// use weapons::Damages;
    ///
    /// let torpedo = Torpedo::new(GuidanceType::Guided, PropulsionType::FuckingSilent);
    /// assert_eq!(torpedo.get_damages(), &Damages::default());
    /// ```
    pub fn get_damages(&self) -> &Damages {
        &self.damages
    }

    /// Get the damages given by the torpedo with a mutable reference
    ///
    /// See Self::get_informations
    pub fn get_damages_mut(&mut self) -> &mut Damages {
        &mut self.damages
    }
}