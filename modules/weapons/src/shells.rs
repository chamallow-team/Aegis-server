use serde::{Deserialize, Serialize};
use crate::WeaponInformations;

/// The type of shell
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd, Copy)]
pub enum ShellType {
    /// Armor-piercing shell, will penetrate armor and explode inside the target
    ///
    /// Very effective against tanks and armored vehicles
    ArmorPiercing = 0,
    /// High-explosive shell, will explode on impact
    ///
    /// Very effective against infantry, lightly armored vehicles and buildings
    HighExplosive = 1,
    /// Fragmentation shell, will explode on impact and send shrapnels everywhere
    ///
    /// Very effective against infantry
    Fragmentation = 2,
    /// High-explosive anti-tank shell, will concentrate the explosion on a small area to penetrate
    /// the armor
    ///
    /// Very effective against tanks and armored vehicles
    HighExplosiveAntiTank = 3,
    /// Armor-piercing discarding sabot shell, will penetrate armor and explode inside the target.
    ///
    /// The sabot is a shell that is used to accelerate the projectile to a very high speed, then
    /// it is discarded and the projectile continues its trajectory at a very high speed.
    ///
    /// Very effective against tanks and armored vehicles
    ArmorPiecingDiscardingSabot = 4,
    /// Armor-piercing fin-stabilized discarding sabot shell, will penetrate armor and explode inside.
    ///
    /// An evolution of the APDS shell, these shells are designed for maximum penetration using a
    /// fin-stabilized dart (sabot) stabilized by fins.
    ///
    /// Very effective against tanks and armored vehicles
    ArmorPiercingFinStabilizedDiscardingSabot = 5,
    /// Tandem charge shell, will penetrate the armor with the first charge, then explode inside.
    ///
    /// These shells use two successive explosive charges. The first charge neutralizes the reactive
    /// armor (if present), while the second charge penetrates the main armor.
    ///
    /// The tandem charge is a modern evolution of the HEAT shell.
    ///
    /// Very effective against tanks and armored vehicles
    TandemCharge = 6,
    /// Mortar shell, will explode in the air and send shrapnels everywhere
    ///
    /// Very effective against infantry
    Mortar = 7,
}


/// A shell is a projectile that is fired by a tank, a cannon, a howitzer or a mortar
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Shell {
    shell_type: ShellType,
    informations: WeaponInformations
}

impl Shell {
    /// Create a new shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let shell = Shell::new(ShellType::ArmorPiercing);
    /// ```
    pub fn new(shell_type: ShellType) -> Self {
        Self {
            shell_type,
            informations: WeaponInformations::default()
        }
    }

    /// Get the information of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let shell = Shell::new(ShellType::ArmorPiercing);
    /// let informations = shell.get_informations();
    /// ```
    pub fn get_informations(&self) -> &WeaponInformations {
        &self.informations
    }

    /// Get the mutable information of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut shell = Shell::new(ShellType::ArmorPiercing);
    /// let informations = shell.get_informations_mut();
    /// ```
    pub fn get_informations_mut(&mut self) -> &mut WeaponInformations {
        &mut self.informations
    }

    /// Get the type of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let shell = Shell::new(ShellType::ArmorPiercing);
    /// let shell_type = shell.get_shell_type();
    /// ```
    pub fn get_shell_type(&self) -> ShellType {
        self.shell_type
    }

    /// Set the type of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut shell = Shell::new(ShellType::ArmorPiercing);
    /// shell.set_shell_type(ShellType::HighExplosive);
    /// ```
    pub fn set_shell_type(&mut self, shell_type: ShellType) {
        self.shell_type = shell_type;
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_shell_default(){
        use super::*;
        let shell = Shell::new(ShellType::ArmorPiercing);
        assert_eq!(shell.get_shell_type(), ShellType::ArmorPiercing);
        assert_eq!(shell.get_informations().name, "".to_string());
        assert_eq!(shell.get_informations().caliber, 0.0);
        assert_eq!(shell.get_informations().damages.building, 0.0);
        assert_eq!(shell.get_informations().damages.infantry, 0.0);
        assert_eq!(shell.get_informations().damages.vehicle, 0.0);
        assert_eq!(shell.get_informations().damages.armored_vehicle, 0.0);
        assert_eq!(shell.get_informations().damages.tank, 0.0);
        assert_eq!(shell.get_informations().damages.helicopter, 0.0);
        assert_eq!(shell.get_informations().damages.plane, 0.0);
        assert_eq!(shell.get_informations().damages.ship, 0.0);
        assert_eq!(shell.get_informations().damages.submarine, 0.0);
        assert_eq!(shell.get_informations().damages.missile, 0.0);
        assert_eq!(shell.get_informations().damages.satellite, 0.0);
        assert_eq!(shell.get_informations().speed, 0.0);
        assert_eq!(shell.get_informations().range, 0.0);
        assert_eq!(shell.get_informations().fire_rate, 0.0);
    }

    #[test]
    fn test_get_shell_type(){
        use super::*;
        let shell = Shell::new(ShellType::ArmorPiercing);
        assert_eq!(shell.get_shell_type(), ShellType::ArmorPiercing);
    }

    #[test]
    fn test_shell_set_shell_type(){
        use super::*;
        let mut shell = Shell::new(ShellType::ArmorPiercing);
        shell.set_shell_type(ShellType::HighExplosive);
        assert_eq!(shell.get_shell_type(), ShellType::HighExplosive);
    }

    #[test]
    fn test_shell_get_informations(){
        use super::*;
        let shell = Shell::new(ShellType::ArmorPiercing);
        assert_eq!(shell.get_informations().name, "".to_string());
        assert_eq!(shell.get_informations().caliber, 0.0);
        assert_eq!(shell.get_informations().damages.building, 0.0);
        assert_eq!(shell.get_informations().damages.infantry, 0.0);
        assert_eq!(shell.get_informations().damages.vehicle, 0.0);
        assert_eq!(shell.get_informations().damages.armored_vehicle, 0.0);
        assert_eq!(shell.get_informations().damages.tank, 0.0);
        assert_eq!(shell.get_informations().damages.helicopter, 0.0);
        assert_eq!(shell.get_informations().damages.plane, 0.0);
        assert_eq!(shell.get_informations().damages.ship, 0.0);
        assert_eq!(shell.get_informations().damages.submarine, 0.0);
        assert_eq!(shell.get_informations().damages.missile, 0.0);
        assert_eq!(shell.get_informations().damages.satellite, 0.0);
        assert_eq!(shell.get_informations().speed, 0.0);
        assert_eq!(shell.get_informations().range, 0.0);
        assert_eq!(shell.get_informations().fire_rate, 0.0);
    }

    #[test]
    fn test_set_shell_informations(){
        use super::*;

        let mut shell = Shell::new(ShellType::Fragmentation);
        shell.get_informations_mut().name = "Caesar 155mm".to_string();
        assert_eq!(shell.get_informations().name, "Caesar 155mm".to_string());
    }
}