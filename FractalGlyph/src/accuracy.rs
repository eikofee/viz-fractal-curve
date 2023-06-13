use super::reader::SampleRecord;
use rayon::prelude::*;

pub fn compute(samples: &[SampleRecord]) -> f64 {
	samples.par_iter()
	.map(
		|s| if s.groundtruth == s.prediction {
			1
		}
		else {
			0
		}
	)
	.sum::<usize>() as f64
	/ samples.len() as f64
	*100.
}


#[cfg(test)]
mod test_accuracy {
	use super::*;
	use super::super::reader::SampleRecord;

	#[test]
	fn test_accuracy_perfect() {

		let db = [
			SampleRecord {
				id: 0,
				groundtruth: 1,
				prediction: 1,
				order: 0,
			},

			SampleRecord {
				id: 1,
				groundtruth: 10,
				prediction: 10,
				order: 2,
			},

			SampleRecord {
				id: 2,
				groundtruth: 2,
				prediction: 2,
				order: 3,
			},

		];

		assert_eq!(
			100.0,
			compute(&db)
		);
		
	}

	#[test]
	fn test_accuracy_bad() {

		let db = [
			SampleRecord {
				id: 0,
				groundtruth: 1,
				prediction: 10,
				order: 0,
			},

			SampleRecord {
				id: 1,
				groundtruth: 10,
				prediction: 1,
				order: 2,
			},

			SampleRecord {
				id: 2,
				groundtruth: 2,
				prediction: 1,
				order: 3,
			},

		];

		assert_eq!(
			0.0,
			compute(&db)
		);
		
	}

	#[test]
	fn test_accuracy() {

		let db = [
			SampleRecord {
				id: 0,
				groundtruth: 1,
				prediction: 10,
				order: 0,
			},

			SampleRecord {
				id: 1,
				groundtruth: 1,
				prediction: 1,
				order: 2,
			},

			SampleRecord {
				id: 2,
				groundtruth: 2,
				prediction: 1,
				order: 3,
			},

		];

		assert_eq!(
			1.0/3.0 * 100.0,
			compute(&db)
		);
		
	}
}