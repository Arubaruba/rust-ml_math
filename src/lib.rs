/// Stores a mean and other necessary state for it to be updated with additional values
/// # Examples
/// ```
/// use ml_math::MeanIncrementor;
/// // Initialize the incrementor
/// let mut mean_inc = MeanIncrementor::new();
/// // Add some values
///	mean_inc.add(0f32);
///	mean_inc.add(1f32);
/// // Get the mean
///	assert_eq!(0.5f32, mean_inc.mean);
/// // Add more values
///	mean_inc.add(1f32);
///	mean_inc.add(2f32);
/// // Get the updated mean
///	assert_eq!(0.5f32, mean_inc.mean);
/// ```
pub struct MeanIncrementor {
	mean: f32,
	num_existing_values: u32
}

impl MeanIncrementor {
	pub fn new() -> MeanIncrementor {
		MeanIncrementor {mean: 0f32, num_existing_values: 0}
	}
	
	/// Update the mean with another value whose weight will be determined by the number of previously added values
	pub fn add(&mut self, value: f32) {
		if self.num_existing_values == 0 {
			// If we have no values yet the mean is simply the first value
			self.mean = value;
		} else {
            let weight_per_value = 1f32 / (self.num_existing_values + 1) as f32;
            self.mean = self.mean * (1.0 - weight_per_value)  + value * weight_per_value;
		}
		self.num_existing_values += 1;
	}
}

#[test] 
fn test_mean_incrementor() {
	let mut mean_inc = MeanIncrementor::new();

	mean_inc.add(0f32);
	assert_eq!(0f32, mean_inc.mean);

	mean_inc.add(1f32);
	assert_eq!(0.5f32, mean_inc.mean);
}

/// Stores a variance and other necessary state for it to be updated with additional values.
/// [See Details on the Formula](http://math.stackexchange.com/questions/102978/incremental-computation-of-standard-deviation)
/// # Examples
/// ```
/// use ml_math::VarianceIncrementor;
/// // Initialize the incrementor
/// let mut mean_inc = VarianceIncrementor::new();
/// // Add some values
///	mean_inc.add(0f32);
///	mean_inc.add(1f32);
///	let mut variance_inc = VarianceIncrementor::new();
/// // Add some values
///	variance_inc.add(0f32);
///	variance_inc.add(1f32);
/// // Get the variance
///	assert_eq!(0.5f32, variance_inc.variance);
/// // Add more values
///	variance_inc.add(2f32);
/// // Get the updated variance
///	assert_eq!(1f32, variance_inc.variance);
/// ```
pub struct VarianceIncrementor {
	variance: f32,
	mean_incrementor: MeanIncrementor
}

impl VarianceIncrementor {
	fn new() -> VarianceIncrementor {
        VarianceIncrementor {variance: 0f32, mean_incrementor: MeanIncrementor::new()}
	}
	
	fn add(&mut self, value: f32) {
		let n = self.mean_incrementor.num_existing_values;
        let previous_mean = self.mean_incrementor.mean;
        self.mean_incrementor.add(value);

		if n == 0 {
			self.variance = 0f32;
        } else {
        	self.variance = (n - 1) as f32 / n as f32 * self.variance + (value - previous_mean).powi(2) / (n + 1) as f32;
		}
	}
}

#[test] 
fn variance_incrementor() {
	let mut variance_inc = VarianceIncrementor::new();

	variance_inc.add(0f32);
	variance_inc.add(1f32);
	assert_eq!(0.5f32, variance_inc.variance);

	variance_inc.add(2f32);
	assert_eq!(1f32, variance_inc.variance);
}