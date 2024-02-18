use serde::{Deserialize, Serialize};
use crate::WeaponID;
use crate::{Damages, WeaponInformations};

/// Enumeration representing different types of firearms.
#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
#[repr(u8)]
pub enum FireArmType {
    /// Hand-held firearm used at short distances, typically operated with one hand.
    ///
    /// Can be semi-automatic or manually operated for repeated shots.
    Gun = 0,
    /// Firearm with a long barrel designed to be held against the shoulder.
    ///
    /// Capable of single-shot, manual repetition, semi-automatic, or automatic firing.
    Rifle = 1,
    /// Firearm designed to fire bursts of bullets.
    SubMachineGun = 2,
    /// Firearm designed for selective fire (semi-automatic or automatic).
    Assault = 3,
    /// Automatic firearm built to fire a continuous stream of bullets.
    MachineGun = 4,
    /// Rifle designed for accurate shooting at long distances.
    PrecisionRifle = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct FireArm {
    /// Contain a list of IDs to get the allowed bullets
    allowed_bullets: Vec<WeaponID>,
    fire_arm_type: FireArmType,
    default_bullets: WeaponID,

    informations: WeaponInformations,
    damages: Damages
}

impl FireArm {
    /// Create a new firearm object
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let firearm = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// assert_eq!(firearm.get_type(), FireArmType::Assault);
    /// ```
    pub fn new(fire_arm_type: FireArmType, default_bullets: impl Into<WeaponID>) -> Self {
        Self {
            fire_arm_type,
            default_bullets: default_bullets.into(),
            informations: WeaponInformations::default(),
            damages: Damages::default(),
            allowed_bullets: Vec::default()
        }
    }

    /// Get the type of the firearm
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let firearm = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// assert_eq!(firearm.get_type(), FireArmType::Assault);
    /// ```
    pub fn get_type(&self) -> FireArmType {
        self.fire_arm_type
    }

    /// Define a new type for the firearm
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let mut firearm = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// firearm.set_type(FireArmType::MachineGun);
    /// assert_eq!(firearm.get_type(), FireArmType::MachineGun);
    /// ```
    pub fn set_type(&mut self, new_type: FireArmType) {
        self.fire_arm_type = new_type;
    }


    /// Get the default bullet defined for this weapon
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let default = "abc".to_string();
    ///
    /// let firearm = FireArm::new(FireArmType::Assault, default.clone());
    /// assert_eq!(firearm.get_default_bullet(), &default);
    /// ```
    pub fn get_default_bullet(&self) -> &WeaponID {
        &self.default_bullets
    }

    /// Define a new default bullet for this weapon
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let default = "abc".to_string();
    ///
    /// let mut firearm = FireArm::new(FireArmType::Assault, default.clone());
    /// assert_eq!(firearm.get_default_bullet(), &default);
    ///
    /// let default2 = "abc".to_string();
    ///
    /// firearm.set_default_bullet(default2.clone());
    /// assert_eq!(firearm.get_default_bullet(), &default2);
    /// ```
    pub fn set_default_bullet(&mut self, new_type: impl Into<WeaponID>) {
        self.default_bullets = new_type.into();
    }

    /// Get the list of allowed bullets for this weapon
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    /// use weapons::WeaponID;
    ///
    /// let firearm = FireArm::new(FireArmType::Assault, "Famas");
    /// assert_eq!(firearm.get_allowed_bullets(), &Vec::<WeaponID>::new());
    /// ```
    pub fn get_allowed_bullets(&self) -> &Vec<WeaponID> {
        &self.allowed_bullets
    }

    /// Get the list of allowed bullets for this weapon with a mutable reference
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    /// use weapons::WeaponID;
    ///
    /// let mut firearm = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// assert_eq!(firearm.get_allowed_bullets_mut(), &mut Vec::<WeaponID>::new());
    /// ```
    pub fn get_allowed_bullets_mut(&mut self) -> &mut Vec<WeaponID> {
        &mut self.allowed_bullets
    }

    /// Add a new allowed bullet for this firearm
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    ///
    /// let mut firearm = FireArm::new(FireArmType::Assault, "Famas".to_string());
    /// let allowed_bullet = "abc".to_string();
    /// firearm.add_allowed_bullet(allowed_bullet.clone());
    /// assert_eq!(firearm.get_allowed_bullets(), &vec![allowed_bullet]);
    /// ```
    pub fn add_allowed_bullet(&mut self, id: impl Into<WeaponID>) {
        let i = id.into();
        if !self.allowed_bullets.contains(&i) {
            self.allowed_bullets.push(i)
        }
    }

    /// Remove a bullet for this firearm
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    /// use weapons::WeaponID;
    ///
    /// let mut firearm = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// let allowed_bullet = "abc".to_string();
    ///
    /// firearm.add_allowed_bullet(allowed_bullet.clone());
    /// assert_eq!(firearm.get_allowed_bullets(), &vec![allowed_bullet.clone()]);
    ///
    /// firearm.remove_allowed_bullet(allowed_bullet);
    /// assert_eq!(firearm.get_allowed_bullets(), &Vec::<WeaponID>::new());
    /// ```
    pub fn remove_allowed_bullet(&mut self, id: impl Into<WeaponID>) {
        let a = id.into();
        self.allowed_bullets.retain(|i| i != &a)
    }

    /// Get the information on the firearm
    ///
    /// # Example
    ///
    /// ```
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    /// use weapons::WeaponInformations;
    ///
    /// let rifle = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// assert_eq!(rifle.get_informations(), &WeaponInformations::default());
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
    /// use uuid::Uuid;
    /// use weapons::firearm::{FireArm, FireArmType};
    /// use weapons::{Damages, WeaponInformations};
    ///
    /// let rifle = FireArm::new(FireArmType::Assault, Uuid::new_v4());
    /// assert_eq!(rifle.get_damages(), &Damages::default());
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