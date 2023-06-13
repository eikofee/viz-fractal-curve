///! This module aim at computing the quality of drawings

use crate::prelude::*;

/// Define a quality metric that is compute on an image representation
pub trait ImageQualityMetric {
	fn compute(&self, img: &Image) -> f64;
}


/// Count the number of pixels of the same color in the neighborhood of a pixel
pub struct PixelNeighboorhoodQuality {
	/// Size of lookup (should be odd)
	pub(crate) kernel_size: usize
}

impl ImageQualityMetric for PixelNeighboorhoodQuality {
	fn compute(&self, img: &Image) -> f64 {
		let local: f64 = (0..img.width()).into_iter().step_by(self.kernel_size).zip( (0..img.height()).into_iter().step_by(self.kernel_size))
			.map(|(x,y)| self.nb_similar_colors(img, x, y) as f64)
			.sum();

		let max = self.max_quality(img);
		let min = self.min_quality(img);
		let width = max - min;
		(local - min)/max
	}


}


impl PixelNeighboorhoodQuality {
	fn nb_similar_colors(&self, img: &Image, x: u32, y: u32) -> usize {
		let color = img.get_pixel(x, y).clone();
		self.coords_for(img, x, y)
			.map(|(x,y)| {
				if color == img.get_pixel(x, y).clone() {
					1
				}
				else {
					0
				}
			})
			.sum()

	}

	/// Check if the coord is within the image and is not black (means no pixel written)
	fn coord_valid_for(img: &Image, x: i64, y: i64) -> bool {
		if x  >= 0 && x < img.width() as i64 && y >=0 && y < img.height() as i64 {
			let color = img.get_pixel(x as u32, y as u32);
			*color != image::Rgb::<u8>([0,0,0])
		}
		else {
			false
		}
	}

	/// For a given coordinates, provides the other coordinates to browse in order to find a matching color
	/// - should be within the image
	/// - should not be centered
	fn coords_for<'a>(&self, img: &'a Image, x: u32, y: u32) -> impl Iterator<Item=(u32, u32)> +'a {

		((-(self.kernel_size as i64-1)/2)..(self.kernel_size as i64-1)/2).into_iter().zip(((-(self.kernel_size as i64 -1)/2)..(self.kernel_size as i64-1)/2).into_iter())
		.filter(|(dx, dy)| {
			*dx != 0 && *dy != 0 // remove center
		})
		.map(move |(dx,dy)|{
			(x as i64 + dx, y as i64 + dy) // compute absolute coordinate
		})
		.filter(move |(lx,ly)| {
			Self::coord_valid_for(img, *lx, *ly) // check if it is within the image
		})
		.map(|(lx, ly)| {
			(lx as u32, ly as u32) // Convert to the right type
		})
	}

	fn max_quality(&self, img: &Image) -> f64 {
		(0..img.width()).into_iter().step_by(self.kernel_size).zip( (0..img.height()).into_iter().step_by(self.kernel_size))
		.map(|(x,y)| {
			self.coords_for(img, x, y).count()
		})
		.sum::<usize>() as f64
	}

	fn min_quality(&self, img: &Image) -> f64 {
		0.0
	}
}