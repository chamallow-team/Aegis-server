//! This module is used to define bullets

use crate::{Damages, WeaponInformations};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize, Deserialize)]
#[repr(u8)]
/// Enumeration representing different types of bullets.
pub enum BulletType {
    /// Full Metal Jacket (FMJ): The bullet is encased in a metallic jacket to enhance penetration and reduce barrel fouling.
    Ordinary = 0,
    /// Hollow Point (HP): The bullet is designed to deform upon impact, causing expansion and resulting in injuries to the target.
    Expansible = 1,
    /// Armor-Piercing (AP): The bullet is reinforced to penetrate materials such as light armor.
    ArmorPiercing = 2,
    /// Frangible Bullets: Designed to reduce the risk of over-penetration, often used in environments where the safety of the surroundings is crucial.
    Frangible = 3,
    /// Tracing Bullets: Contains a pyrotechnic mix producing a visible light trail, allowing soldiers to track the trajectory of the bullet.
    ///
    /// Often used for position reporting and trajectory correction.
    Tracing = 4,
    /// Armor-Piercing Incendiary (API): Combination of a perforating point and a flammable material.
    ///
    /// Used against armored targets and can cause fires.
    ArmorPiercingIncendiary = 5,
    /// Saboted Light Armor Penetrator (SLAP): A small bullet with a sabot that detaches after the shot, enabling the small bullet to penetrate armored targets.
    SabotedLightArmorPenetrator = 6,
    /// Fragmentation Bullet: Designed to fragment into small pieces upon impact, increasing injuries inflicted on the target.
    Fragmentation = 7,
}

impl TryFrom<i64> for BulletType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Ordinary),
            1 => Ok(Self::Expansible),
            2 => Ok(Self::ArmorPiercing),
            3 => Ok(Self::Frangible),
            4 => Ok(Self::Tracing),
            5 => Ok(Self::ArmorPiercingIncendiary),
            6 => Ok(Self::SabotedLightArmorPenetrator),
            7 => Ok(Self::Fragmentation),
            _ => Err(()),
        }
    }
}

/// Implement a bullet
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Bullet {
    bullet_type: BulletType,

    informations: WeaponInformations,
    damages: Damages,
}

impl Bullet {
    /// Create a new bullet with his type
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::bullets::{Bullet, BulletType};
    ///
    /// let bullet = Bullet::new(BulletType::ArmorPiercing);
    /// assert_eq!(bullet.get_bullet_type(), BulletType::ArmorPiercing);
    /// ```
    pub fn new(bullet_type: BulletType) -> Self {
        Self {
            bullet_type,

            informations: WeaponInformations::default(),
            damages: Damages::default(),
        }
    }

    /// Get the type of the buller
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::bullets::{Bullet, BulletType};
    ///
    /// let bullet = Bullet::new(BulletType::ArmorPiercing);
    /// assert_eq!(bullet.get_bullet_type(), BulletType::ArmorPiercing);
    /// ```
    pub fn get_bullet_type(&self) -> BulletType {
        self.bullet_type
    }

    /// Define a new type for the bullet
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::bullets::{Bullet, BulletType};
    ///
    /// let mut bullet = Bullet::new(BulletType::ArmorPiercing);
    /// assert_eq!(bullet.get_bullet_type(), BulletType::ArmorPiercing);
    /// bullet.set_bullet_type(BulletType::Tracing);
    /// assert_eq!(bullet.get_bullet_type(), BulletType::Tracing);
    /// ```
    pub fn set_bullet_type(&mut self, bullet_type: BulletType) {
        self.bullet_type = bullet_type;
    }

    /// Get the information on the torpedo
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::bullets::{Bullet, BulletType};
    /// use weapons::WeaponInformations;
    ///
    /// let bullet = Bullet::new(BulletType::Ordinary);
    /// assert_eq!(bullet.get_informations(), &WeaponInformations::default());
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

    /// Get the damages given by the bullet
    ///
    /// # Example
    ///
    /// ```
    /// use weapons::bullets::{Bullet, BulletType};
    /// use weapons::Damages;
    ///
    /// let bullet = Bullet::new(BulletType::Ordinary);
    /// assert_eq!(bullet.get_damages(), &Damages::default());
    /// ```
    pub fn get_damages(&self) -> &Damages {
        &self.damages
    }

    /// Get the damages given by the bullet with a mutable reference
    ///
    /// See Self::get_informations
    pub fn get_damages_mut(&mut self) -> &mut Damages {
        &mut self.damages
    }
}
