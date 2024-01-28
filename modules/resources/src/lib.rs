use std::fmt::Display;

pub trait Resource: Clone + Display + Sized + Sync {}


/// Contain an amount of food
///
/// This amount can go from 0 to infinity
///
/// # Examples
/// ```rs
/// let mut food = Food::default();
/// food.add(10);
/// assert_eq!(food.get(), 10);
/// ```
#[derive(Clone, Default)]
pub struct Food {
    amount: u64,
}

impl Food {
    /// Create a new Food with an amount
    ///
    /// # Examples
    /// ```rs
    /// let food = Food::new(10);
    /// assert_eq!(food.get(), 10);
    /// ```
    fn new(amount: u64) -> Self {
        Self {
            amount
        }
    }
    /// Add an amount of food
    ///
    /// # Examples
    /// ```rs
    /// let mut food = Food::default();
    /// food.add(10);
    /// assert_eq!(food.get(), 10);
    /// ```
    fn add(&mut self, amount: u64) {
        self.amount += amount;
    }
    /// Remove an amount of food
    ///
    /// Return false if the amount of food is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut food = Food::new(10);
    /// let r = food.remove(5);
    /// assert_eq!(food.get(), 5);
    /// assert!(r);
    /// ```
    ///
    /// ```rs
    /// let mut food = Food::new(10);
    /// let r = food.remove(20);
    /// assert_eq!(food.get(), 20);
    /// assert!(!r);
    /// ```
    fn remove(&mut self, amount: u64) -> bool {
        if self.amount < amount {
            return false;
        }
        self.amount -= amount;
        true
    }
    /// Get the amount of food
    fn get(&self) -> u64 {
        self.amount
    }
}
impl Display for Food {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Food({})", self.amount)
    }
}
impl Resource for Food {}

/// Contain an amount of money
///
/// This amount can go from -infinity to infinity
///
/// # Examples
/// ```rs
/// let mut money = Money::default();
/// money.add(10);
/// assert_eq!(money.get(), 10);
/// ```
#[derive(Clone, Default)]
pub struct Money {
    amount: i64
}

impl Money {
    /// Create new Money with an amount
    ///
    /// # Examples
    /// ```rs
    /// let money = Money::new(10);
    /// assert_eq!(money.get(), 10);
    /// ```
    fn new(amount: i64) -> Self {
        Self {
            amount
        }
    }
    /// Add an amount of money
    ///
    /// # Examples
    /// ```rs
    /// let mut money = Money::default();
    /// money.add(10);
    /// assert_eq!(money.get(), 10);
    /// ```
    fn add(&mut self, amount: i64) {
        self.amount += amount;
    }
    /// Remove an amount of money
    ///
    /// Return false if the amount of money is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut money = Money::new(10);
    /// let r = money.remove(5);
    /// assert_eq!(money.get(), 5);
    /// assert!(r);
    /// ```
    ///
    /// ```rs
    /// let mut money = Money::new(10);
    /// let r = money.remove(20);
    /// assert_eq!(money.get(), 20);
    /// assert!(!r);
    fn remove(&mut self, amount: i64) -> bool {
        self.amount -= amount;
        true
    }
    /// Return true if the amount of money is negative
    /// # Examples
    /// ```rs
    /// let mut money = Money::new(10);
    /// assert!(!money.is_negative());
    /// ```
    fn is_negative(&self) -> bool {
        self.amount < 0
    }
    /// Get the amount of money
    ///
    /// # Examples
    /// ```rs
    /// let mut money = Money::new(10);
    /// assert_eq!(money.get(), 10);
    /// ```
    fn get(&self) -> i64 {
        self.amount
    }
}
impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Money({})", self.amount)
    }
}
impl Resource for Money {}

/// Contain an amount of work force
///
/// This amount can go from 0 to infinity
///
/// # Examples
/// ```rs
/// let mut work_force = WorkForce::default();
/// work_force.add(10);
/// assert_eq!(work_force.get(), 10);
/// ```
#[derive(Clone, Default)]
pub struct WorkForce {
    amount: u64
}

impl WorkForce {
    /// Create new WorkForce with an amount
    ///
    /// # Examples
    /// ```rs
    /// let work_force = WorkForce::new(10);
    /// assert_eq!(work_force.get(), 10);
    /// ```
    fn new(amount: u64) -> Self {
        Self {
            amount
        }
    }
    /// Add an amount of work force
    ///
    /// # Examples
    /// ```rs
    /// let mut work_force = WorkForce::default();
    /// work_force.add(10);
    /// assert_eq!(work_force.get(), 10);
    /// ```
    fn add(&mut self, amount: u64) {
        self.amount += amount;
    }
    /// Remove an amount of work force
    ///
    /// Return false if the amount of work force is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut work_force = WorkForce::new(10);
    /// let r = work_force.remove(5);
    /// assert_eq!(work_force.get(), 5);
    /// assert!(r);
    /// ```
    ///
    /// ```rs
    /// let mut work_force = WorkForce::new(10);
    /// let r = work_force.remove(20);
    /// assert_eq!(work_force.get(), 20);
    /// assert!(!r);
    /// ```
    fn remove(&mut self, amount: u64) -> bool {
        if self.amount < amount {
            return false;
        }
        self.amount -= amount;
        true
    }
    /// Get the amount of work force
    ///
    /// # Examples
    /// ```rs
    /// let mut work_force = WorkForce::new(10);
    /// assert_eq!(work_force.get(), 10);
    /// ```
    fn get(&self) -> u64 {
        self.amount
    }
}
impl Display for WorkForce {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "WorkForce({})", self.amount)
    }
}
impl Resource for WorkForce {}

/// Contain an amount of ores
///
/// This amount can go from 0 to infinity
///
/// # Examples
/// ```rs
/// let mut ores = Ores::default();
/// ores.add(10);
/// assert_eq!(ores.get(), 10);
/// ```
#[derive(Clone, Default)]
pub struct Ores {
    uranium: u64,
    rate_metals: u64
}

impl Ores {
    /// Create new Ores with an amount
    ///
    /// # Examples
    /// ```rs
    /// let ores = Ores::new(10, 20);
    /// assert_eq!(ores.get_uranium(), 10);
    /// assert_eq!(ores.get_rate_metals(), 20);
    /// ```
    fn new(uranium: u64, rate_metals: u64) -> Self {
        Self {
            uranium,
            rate_metals
        }
    }
    /// Add an amount of uranium
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::default();
    /// ores.add_uranium(10);
    /// assert_eq!(ores.get_uranium(), 10);
    /// ```
    fn add_uranium(&mut self, amount: u64) {
        self.uranium += amount;
    }
    /// Add an amount of rate metals
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::default();
    /// ores.add_rate_metals(10);
    /// assert_eq!(ores.get_rate_metals(), 10);
    /// ```
    fn add_rate_metals(&mut self, amount: u64) {
        self.rate_metals += amount;
    }
    /// Remove an amount of uranium
    ///
    /// Return false if the amount of uranium is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// let r = ores.remove_uranium(5);
    /// assert_eq!(ores.get_uranium(), 5);
    /// assert!(r);
    /// ```
    ///
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// let r = ores.remove_uranium(20);
    /// assert_eq!(ores.get_uranium(), 20);
    /// assert!(!r);
    /// ```
    fn remove_uranium(&mut self, amount: u64) -> bool {
        if self.uranium < amount {
            return false;
        }
        self.uranium -= amount;
        true
    }
    /// Remove an amount of rate metals
    ///
    /// Return false if the amount of rate metals is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// let r = ores.remove_rate_metals(5);
    /// assert_eq!(ores.get_rate_metals(), 5);
    /// assert!(r);
    /// ```
    ///
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// let r = ores.remove_rate_metals(20);
    /// assert_eq!(ores.get_rate_metals(), 20);
    /// assert!(!r);
    /// ```
    fn remove_rate_metals(&mut self, amount: u64) -> bool {
        if self.rate_metals < amount {
            return false;
        }
        self.rate_metals -= amount;
        true
    }
    /// Get the amount of uranium
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// assert_eq!(ores.get_uranium(), 10);
    /// ```
    fn get_uranium(&self) -> u64 {
        self.uranium
    }
    /// Get the amount of rate metals
    ///
    /// # Examples
    /// ```rs
    /// let mut ores = Ores::new(10, 20);
    /// assert_eq!(ores.get_rate_metals(), 20);
    /// ```
    fn get_rate_metals(&self) -> u64 {
        self.rate_metals
    }
}
impl Display for Ores {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Ores({} {})", self.uranium, self.rate_metals)
    }
}
impl Resource for Ores {}

#[derive(Clone, Default)]
pub struct RefinedProduct {
    alloys: u64,
    chips: u64,
    components: u64
}

impl RefinedProduct {
    /// Create new RefinedProduct with an amount
    ///
    /// # Examples
    /// ```rs
    /// let refined_product = RefinedProduct::new(10, 20, 30);
    /// assert_eq!(refined_product.get_alloys(), 10);
    /// assert_eq!(refined_product.get_chips(), 20);
    /// assert_eq!(refined_product.get_components(), 30);
    /// ```
    fn add_alloys(&mut self, amount: u64) {
        self.alloys += amount;
    }
    /// Add an amount of chips
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::default();
    /// refined_product.add_chips(10);
    /// assert_eq!(refined_product.get_chips(), 10);
    /// ```
    fn add_chips(&mut self, amount: u64) {
        self.chips += amount;
    }
    /// Add an amount of components
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::default();
    /// refined_product.add_components(10);
    /// assert_eq!(refined_product.get_components(), 10);
    /// ```
    fn add_components(&mut self, amount: u64) {
        self.components += amount;
    }
    /// Remove an amount of alloys
    ///
    /// Return false if the amount of alloys is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// let r = refined_product.remove_alloys(5);
    /// assert_eq!(refined_product.get_alloys(), 5);
    /// assert!(r);
    /// ```
    fn remove_alloys(&mut self, amount: u64) -> bool {
        if self.alloys < amount {
            return false;
        }
        self.alloys -= amount;
        true
    }
    /// Remove an amount of chips
    ///
    /// Return false if the amount of chips is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// let r = refined_product.remove_chips(5);
    /// assert_eq!(refined_product.get_chips(), 5);
    /// assert!(r);
    /// ```
    fn remove_chips(&mut self, amount: u64) -> bool {
        if self.chips < amount {
            return false;
        }
        self.chips -= amount;
        true
    }
    /// Remove an amount of components
    ///
    /// Return false if the amount of components is not enough
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// let r = refined_product.remove_components(5);
    /// assert_eq!(refined_product.get_components(), 5);
    /// assert!(r);
    /// ```
    fn remove_components(&mut self, amount: u64) -> bool {
        if self.components < amount {
            return false;
        }
        self.components -= amount;
        true
    }
    /// Get the amount of alloys
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// assert_eq!(refined_product.get_alloys(), 10);
    /// ```
    fn get_alloys(&self) -> u64 {
        self.alloys
    }
    /// Get the amount of chips
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// assert_eq!(refined_product.get_chips(), 20);
    /// ```
    fn get_chips(&self) -> u64 {
        self.chips
    }
    /// Get the amount of components
    ///
    /// # Examples
    /// ```rs
    /// let mut refined_product = RefinedProduct::new(10, 20, 30);
    /// assert_eq!(refined_product.get_components(), 30);
    /// ```
    fn get_components(&self) -> u64 {
        self.components
    }
}
impl Display for RefinedProduct {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "RefinedProduct({} {} {})", self.alloys, self.chips, self.components)
    }
}
impl Resource for RefinedProduct {}

/// Contain the amount of scientific research and the number of experts
///
/// This amount can go from 0 to infinity
///
/// # Examples
/// ```rs
/// let mut scientific_research = ScientificResearch::default();
/// scientific_research.add(10);
/// assert_eq!(scientific_research.get(), 10);
/// ```
#[derive(Clone, Default)]
pub struct ScientificResearch {
    experts: Vec<Expert>
}

impl ScientificResearch {
    /// Create new ScientificResearch with an amount
    ///
    /// # Examples
    /// ```rs
    /// let scientific_research = ScientificResearch::default();
    /// assert_eq!(scientific_research.size(), 0);
    /// ```
    /// ```rs
    /// let scientific_research = ScientificResearch::new(10);
    /// scientific_research.add_expert(Expert::new(10));
    /// assert_eq!(scientific_research.size(), 1);
    /// ```
    pub fn add_expert(&mut self, expert: Expert) {
        self.experts.push(expert);
    }

    /// Add an amount of scientific research
    ///
    /// # Examples
    /// ```rs
    /// let mut scientific_research = ScientificResearch::default();
    /// assert_eq!(scientific_research.size(), 0);
    /// ```
    ///
    /// ```rs
    /// let mut scientific_research = ScientificResearch::new(10);
    /// scientific_research.add_expert(Expert::new(10));
    /// assert_eq!(scientific_research.size(), 1);
    /// ```
    pub fn size(&self) -> usize {
        self.experts.len()
    }

    /// Add an amount of scientific research
    ///
    /// # Examples
    /// ```rs
    /// let mut scientific_research = ScientificResearch::default();
    /// scientific_research.add(Expert::new(10));
    /// scientific_research.add(Expert::new(20));
    ///
    /// assert_eq!(scientific_research.get_amount(), 30);
    /// ```
    pub fn get_amount(&self) -> u64 {
        self.experts.iter()
            .fold(0, |acc, expert| acc + expert.get_level() as u64)
    }
}
impl Display for ScientificResearch {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut experts = String::new();
        for expert in &self.experts {
            experts.push_str(&expert.to_string());
            experts.push_str(" ");
        }
        write!(f, "ScientificResearch({})", experts)
    }
}
impl Resource for ScientificResearch {}

/// Represent a scientific expert
///
/// # Examples
/// ```rs
/// let expert = Expert::new(10);
/// assert_eq!(expert.get_level(), 10);
/// ```
#[derive(Clone)]
pub struct Expert {
    level: u8
}

impl Expert {
    /// Create a new Expert with a level
    ///
    /// # Examples
    /// ```rs
    /// let expert = Expert::new(10);
    /// assert_eq!(expert.get_level(), 10);
    /// ```
    fn new(level: u8) -> Self {
        Self { level }
    }
    /// Get the level of the expert
    ///
    /// # Examples
    /// ```rs
    /// let expert = Expert::new(10);
    /// assert_eq!(expert.get_level(), 10);
    /// ```
    fn get_level(&self) -> u8 {
        self.level
    }
}
impl Display for Expert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.level)
    }
}


#[cfg(test)]
mod resources_test {
    #[test]
    fn food(){
        use super::Food;

        let mut food = Food::default();
        food.add(10);
        assert_eq!(food.get(), 10);

        let mut r: bool;

        r = food.remove(5);
        assert_eq!(food.get(), 5);
        assert!(r);
        r = food.remove(10);
        assert_eq!(food.get(), 5);
        assert!(!r);
    }

    #[test]
    fn money(){
        use super::Money;

        let mut money = Money::default();
        money.add(10);
        assert_eq!(money.get(), 10);
        money.remove(5);
        assert_eq!(money.get(), 5);
        money.remove(10);
        assert_eq!(money.get(), -5);
        assert!(money.is_negative());
    }

    #[test]
    fn work_force(){
        use super::WorkForce;

        let mut work_force = WorkForce::default();
        work_force.add(10);
        assert_eq!(work_force.get(), 10);

        let mut r: bool;

        r = work_force.remove(5);
        assert_eq!(work_force.get(), 5);
        assert!(r);
        r = work_force.remove(10);
        assert_eq!(work_force.get(), 5);
        assert!(!r);
    }

    #[test]
    fn ores(){
        use super::Ores;

        let mut ores = Ores::default();
        ores.add_uranium(10);
        assert_eq!(ores.get_uranium(), 10);
        ores.add_rate_metals(20);
        assert_eq!(ores.get_rate_metals(), 20);

        let mut r: bool;

        r = ores.remove_uranium(5);
        assert_eq!(ores.get_uranium(), 5);
        assert!(r);
        r = ores.remove_uranium(10);
        assert_eq!(ores.get_uranium(), 5);
        assert!(!r);

        r = ores.remove_rate_metals(5);
        assert_eq!(ores.get_rate_metals(), 15);
        assert!(r);

        r = ores.remove_rate_metals(20);
        assert_eq!(ores.get_rate_metals(), 15);
        assert!(!r);
    }

    #[test]
    fn refined_product(){
        use super::RefinedProduct;

        let mut refined_product = RefinedProduct::default();
        refined_product.add_alloys(10);
        assert_eq!(refined_product.get_alloys(), 10);
        refined_product.add_chips(20);
        assert_eq!(refined_product.get_chips(), 20);
        refined_product.add_components(30);
        assert_eq!(refined_product.get_components(), 30);

        let mut r: bool;

        r = refined_product.remove_alloys(5);
        assert_eq!(refined_product.get_alloys(), 5);
        assert!(r);

        r = refined_product.remove_alloys(10);
        assert_eq!(refined_product.get_alloys(), 5);
        assert!(!r);

        r = refined_product.remove_chips(5);
        assert_eq!(refined_product.get_chips(), 15);
        assert!(r);

        r = refined_product.remove_chips(20);
        assert_eq!(refined_product.get_chips(), 15);
        assert!(!r);

        r = refined_product.remove_components(5);
        assert_eq!(refined_product.get_components(), 25);
        assert!(r);

        r = refined_product.remove_components(30);
        assert_eq!(refined_product.get_components(), 25);
        assert!(!r);
    }

    #[test]
    fn scientific_research(){
        use super::{ScientificResearch, Expert};

        let mut scientific_research = ScientificResearch::default();
        assert_eq!(scientific_research.size(), 0);
        assert_eq!(scientific_research.get_amount(), 0);

        scientific_research.add_expert(Expert::new(10));
        assert_eq!(scientific_research.size(), 1);

        scientific_research.add_expert(Expert::new(20));
        assert_eq!(scientific_research.size(), 2);

        assert_eq!(scientific_research.get_amount(), 30);
    }
}
