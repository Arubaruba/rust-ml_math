/// Stores a mean and other necessary state for it to be updated with additional values
/// # Examples
/// ```
/// use ml_math::MeanIncrementor;
/// // Initialize the incrementor
/// let mut mean_inc = MeanIncrementor::new();
/// // Add some values
///	mean_incr.add(0f64);
///	mean_incr.add(1f64);
/// // Get the mean
///	assert_eq!(0.5f64, mean_incr.mean);
/// // Add more values
///	mean_incr.add(1f64);
///	mean_incr.add(2f64);
/// // Get the updated mean
///	assert_eq!(0.5f64, mean_incr.mean);
/// ```
#[derive(Copy, Clone)]
pub struct MeanIncrementor {
	mean: f64,
	count: u32
}

impl MeanIncrementor {
	pub fn new() -> MeanIncrementor {
		MeanIncrementor {mean: 0f64, count: 0}
	}
	
	/// Update the mean with another value whose weight will be determined by the number of previously added values
	pub fn add(&mut self, value: f64) {
		if self.count == 0 {
			// If we have no values yet the mean is simply the first value
			self.mean = value;
		} else {
            let weight_per_value = 1f64 / (self.count + 1) as f64;
            self.mean = self.mean * (1.0 - weight_per_value)  + value * weight_per_value;
		}
		self.count += 1;
	}
	
	pub fn mean(&self) -> f64 {
		self.mean
	}
	
	pub fn count(&self) -> u32 {
		self.count
	}
}

#[test] 
fn test_mean_incrementor() {
	let mut mean_incr = MeanIncrementor::new();

	mean_incr.add(0f64);
	assert_eq!(0f64, mean_incr.mean());

	mean_incr.add(1f64);
	assert_eq!(0.5f64, mean_incr.mean());
}

/// Stores a variance and other necessary state for it to be updated with additional values.
/// [See Details on the Formula](http://math.stackexchange.com/questions/102978/incremental-computation-of-standard-deviation)
/// # Examples
/// ```
/// use ml_math::VarianceIncrementor;
/// // Initialize the incrementor
///	let mut variance_inc = VarianceIncrementor::new();
/// // Add some values
///	variance_incr.add(0f64);
///	variance_incr.add(1f64);
/// // Get the variance
///	assert_eq!(0.5f64, variance_incr.variance());
/// // Add more values
///	variance_incr.add(2f64);
/// // Get the updated variance
///	assert_eq!(1f64, variance_incr.variance());
/// ```
#[derive(Copy, Clone)]
pub struct VarianceIncrementor {
	variance: f64,
	mean_incrementor: MeanIncrementor
}

impl VarianceIncrementor {
	pub fn new() -> VarianceIncrementor {
        VarianceIncrementor {variance: 0f64, mean_incrementor: MeanIncrementor::new()}
	}
	
	pub fn add(&mut self, value: f64) {
		let n = self.mean_incrementor.count();
        let previous_mean = self.mean_incrementor.mean();
        self.mean_incrementor.add(value);

		if n == 0 {
			self.variance = 0f64;
        } else {
        	self.variance = (n - 1) as f64 / n as f64 * self.variance + (value - previous_mean).powi(2) / (n + 1) as f64;
		}
	}
	
	pub fn variance(&self) -> f64 {
		self.variance
	}
	
	pub fn mean(&self) -> f64 {
		self.mean_incrementor.mean()
	}
	
	pub fn count(&self) -> u32 {
		self.mean_incrementor.count()
	}
}

#[test] 
fn variance_incrementor() {
	let mut variance_incr = VarianceIncrementor::new();

	variance_incr.add(0f64);
	variance_incr.add(1f64);
	assert_eq!(0.5f64, variance_incr.variance());

	variance_incr.add(2f64);
	assert_eq!(1f64, variance_incr.variance());
}