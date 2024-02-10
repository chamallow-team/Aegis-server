use serde::{Deserialize, Serialize};
use crate::{Damages, WeaponInformations};

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

impl TryFrom<i64> for ShellType {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(ShellType::ArmorPiercing),
            1 => Ok(ShellType::HighExplosive),
            2 => Ok(ShellType::Fragmentation),
            3 => Ok(ShellType::HighExplosiveAntiTank),
            4 => Ok(ShellType::ArmorPiecingDiscardingSabot),
            5 => Ok(ShellType::ArmorPiercingFinStabilizedDiscardingSabot),
            6 => Ok(ShellType::TandemCharge),
            7 => Ok(ShellType::Mortar),
            _ => Err(())
        }
    }
}

/// A shell is a projectile that is fired by a tank, a cannon, a howitzer or a mortar
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, PartialOrd)]
pub struct Shell {
    shell_type: ShellType,
    informations: WeaponInformations,
    damages: Damages
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
            informations: WeaponInformations::default(),
            damages: Damages::default()
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

    /// Set the information of the shell
    pub fn set_informations(&mut self, informations: WeaponInformations) {
        self.informations = informations;
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

    /// Get the damages of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let shell = Shell::new(ShellType::ArmorPiercing);
    /// let damages = shell.get_damages();
    /// ```
    pub fn get_damages(&self) -> &Damages {
        &self.damages
    }

    /// Get the mutable damages of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut shell = Shell::new(ShellType::ArmorPiercing);
    /// let damages = shell.get_damages_mut();
    /// ```
    pub fn get_damages_mut(&mut self) -> &mut Damages {
        &mut self.damages
    }

    /// Set the damages of the shell
    ///
    /// # Example
    ///
    /// ```rs
    /// let mut shell = Shell::new(ShellType::ArmorPiercing);
    /// shell.set_damages(Damages::default());
    /// ```
    pub fn set_damages(&mut self, damages: Damages) {
        self.damages = damages;
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
        assert_eq!(shell.get_informations().speed, 0.0);
        assert_eq!(shell.get_informations().range, 0.0);
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
        assert_eq!(shell.get_informations().speed, 0.0);
        assert_eq!(shell.get_informations().range, 0.0);
    }

    #[test]
    fn test_set_shell_informations(){
        use super::*;

        let mut shell = Shell::new(ShellType::Fragmentation);
        shell.get_informations_mut().name = "Caesar 155mm".to_string();
        assert_eq!(shell.get_informations().name, "Caesar 155mm".to_string());
    }

    #[test]
    fn test_shell_get_damages(){
        use super::*;
        let shell = Shell::new(ShellType::ArmorPiercing);
        assert_eq!(shell.get_damages().building, 0.0);
        assert_eq!(shell.get_damages().infantry, 0.0);
        assert_eq!(shell.get_damages().vehicle, 0.0);
        assert_eq!(shell.get_damages().armored_vehicle, 0.0);
        assert_eq!(shell.get_damages().tank, 0.0);
        assert_eq!(shell.get_damages().helicopter, 0.0);
        assert_eq!(shell.get_damages().plane, 0.0);
        assert_eq!(shell.get_damages().ship, 0.0);
        assert_eq!(shell.get_damages().submarine, 0.0);
        assert_eq!(shell.get_damages().missile, 0.0);
        assert_eq!(shell.get_damages().satellite, 0.0);
    }

    #[cfg(feature = "load_configuration")]
    #[test]
    fn load_config(){
        let mut p = env::current_dir().unwrap();
        p.push("../../data/config/weapons");

        let store = crate::loader::load(p).unwrap();

        dbg!(store);
    }
}

#[cfg(feature = "load_configuration")]
pub(crate) mod loader {
    use std::{fs, io};
    use std::path::PathBuf;
    use toml::Table;
    use crate::loader::{parse_damages, parse_weapons_information, WeaponsStore};
    use crate::shells::{Shell, ShellType};

    pub(crate) fn read_shells(dir: PathBuf, store: &mut WeaponsStore) -> io::Result<()> {
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

            if let Some(s) = parse_shell(&dir, v.as_table().unwrap()) {
                store.shell.insert(k.into(), s);
            }
        }

        Ok(())
    }

    fn parse_shell(dir: &PathBuf, t: &Table) -> Option<Shell> {
        if !t.contains_key("shell_type") {
            // TODO use a proper logging method
            println!("\x1b[33mWarning: the shell at {dir:?} does not have a shell_type\x1b[0m");
            return None;
        }

        let shell_type = match t.get("shell_type").unwrap().as_integer() {
            Some(shell_type) => match ShellType::try_from(shell_type) {
                Ok(shell_type) => shell_type,
                Err(_) => {
                    // TODO use a proper logging method
                    println!("\x1b[33mWarning: the shell_type of the shell at {dir:?} is unknown\x1b[0m");
                    return None;
                }
            },
            None => {
                // TODO use a proper logging method
                println!("\x1b[33mWarning: the shell_type of the shell at {dir:?} is not an integer\x1b[0m");
                return None;
            }
        };

        let mut shell = Shell::new(shell_type);

        for (k, v) in t.iter().filter(|(k, _)| k.as_str() != "shell_type") {
            match k.as_str() {
                "informations" => match parse_weapons_information(v) {
                    Ok(t) => shell.set_informations(t),
                    Err(e) => {
                        // TODO use a proper logging method
                        println!("\x1b[33mWarning: cannot parse the informations of the shell at {dir:?}: {e:?}\x1b[0m");
                        return None;
                    }
                },
                "damages" => match parse_damages(v) {
                    Ok(d) => shell.set_damages(d),
                    Err(e) => {
                        // TODO use a proper logging method
                        println!("\x1b[33mWarning: cannot parse the damages of the shell at {dir:?}: {e:?}\x1b[0m");
                        return None;
                    }
                },
                _ => {
                    // TODO use a proper logging method
                    println!("\x1b[33mWarning: the key {k:?} of the shell at {dir:?} is unknown\x1b[0m");
                    continue;
                }
            }
        }

        Some(shell)
    }
}
