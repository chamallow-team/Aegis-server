/// A coefficient is a value that can be used to modify the value of a resource.
///
/// Coefficients are used to modify the value of a resource when it is being
/// produced or consumed. For example, if a resource is being produced at a rate
/// of 1.0 per second, and it has a coefficient of 2.0, then it will be produced
/// at a rate of 2.0 per second.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Coefficient(f64);

impl Coefficient {
    /// Create a new coefficient.
    ///
    /// # Examples
    /// ```rs
    /// let coefficient = Coefficient::new(1.0);
    /// ```
    pub fn new(value: f64) -> Self {
        Self(value)
    }

    /// Get the value of the coefficient.
    ///
    /// # Examples
    /// ```rs
    /// let coefficient = Coefficient::new(1.0);
    /// assert_eq!(coefficient.value(), 1.0);
    /// ```
    pub fn value(&self) -> f64 {
        self.0
    }

    /// Set the value of the coefficient.
    ///
    /// # Examples
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.set_value(2.0);
    /// assert_eq!(coefficient.value(), 2.0);
    /// ```
    pub fn set_value(&mut self, value: f64) {
        self.0 = value;
    }

    /// Add a value to the coefficient.
    ///
    /// # Examples
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.add(2.0);
    /// assert_eq!(coefficient.value(), 3.0);
    /// ```
    pub fn add(&mut self, value: f64) {
        self.0 += value;
    }

    /// Subtract a value from the coefficient.
    ///
    /// If the value to subtract is greater than the coefficient, the coefficient
    /// will be set to 0.0.
    ///
    /// # Examples
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.sub(0.5);
    /// assert_eq!(coefficient.value(), 0.5);
    /// ```
    ///
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.sub(2.0);
    /// assert_eq!(coefficient.value(), 0.0);
    /// ```
    pub fn sub(&mut self, value: f64) {
        if self.0 < value {
            self.0 = 0.0;
        } else {
            self.0 -= value;
        }
    }

    /// Multiply the coefficient by a value.
    ///
    /// # Examples
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.mul(2.0);
    /// assert_eq!(coefficient.value(), 2.0);
    /// ```
    pub fn mul(&mut self, value: f64) {
        self.0 *= value;
    }

    /// Divide the coefficient by a value.
    ///
    /// If the value to divide by is 0.0, the coefficient will not be modified.
    ///
    /// # Examples
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.div(2.0);
    /// assert_eq!(coefficient.value(), 0.5);
    /// ```
    ///
    /// ```rs
    /// let mut coefficient = Coefficient::new(1.0);
    /// coefficient.div(0.0);
    /// assert_eq!(coefficient.value(), 0.0);
    /// ```
    pub fn div(&mut self, value: f64) {
        if value != 0.0 {
            self.0 /= value;
        }
    }
}

impl Default for Coefficient {
    fn default() -> Self {
        Self(1.0)
    }
}

#[cfg(test)]
mod coefficient_test {
    #[test]
    fn new() {
        use super::Coefficient;

        let coefficient = Coefficient::new(1.0);
        assert_eq!(coefficient.value(), 1.0);
    }

    #[test]
    fn set_value() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.set_value(2.0);
        assert_eq!(coefficient.value(), 2.0);
    }

    #[test]
    fn add() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.add(2.0);
        assert_eq!(coefficient.value(), 3.0);
    }

    #[test]
    fn sub() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.sub(0.5);
        assert_eq!(coefficient.value(), 0.5);
    }

    #[test]
    fn sub_zero() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.sub(2.0);
        assert_eq!(coefficient.value(), 0.0);
    }

    #[test]
    fn mul() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.mul(2.0);
        assert_eq!(coefficient.value(), 2.0);
    }

    #[test]
    fn div() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.div(2.0);
        assert_eq!(coefficient.value(), 0.5);
    }

    #[test]
    fn div_zero() {
        use super::Coefficient;

        let mut coefficient = Coefficient::new(1.0);
        coefficient.div(0.0);
        assert_eq!(coefficient.value(), 1.0);
    }

    #[test]
    fn default() {
        use super::Coefficient;

        let coefficient = Coefficient::default();
        assert_eq!(coefficient.value(), 1.0);
    }

    #[test]
    fn partial_eq() {
        use super::Coefficient;

        let coefficient_1 = Coefficient::new(1.0);
        let coefficient_2 = Coefficient::new(1.0);
        assert_eq!(coefficient_1, coefficient_2);
    }

    #[test]
    fn partial_ord() {
        use super::Coefficient;

        let coefficient_1 = Coefficient::new(1.0);
        let coefficient_2 = Coefficient::new(2.0);
        assert!(coefficient_1 < coefficient_2);
    }
}