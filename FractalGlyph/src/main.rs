///! The aim of fractal view is to build miniature on a fractal curve of ordered samples
mod reader;
mod barycenter;
mod gosper;
mod prelude;
mod quality;
mod accuracy;

use hilbert::transform::fast_hilbert::hilbert_axes;
use image;
use image::Rgb;
use itertools::Itertools;

use crate::prelude::*;


/// A sample corresponds to a data associated to an order and class
#[derive(Clone, Copy, Debug)]
pub struct Sample {
    pub(crate) label: LabelId,
    pub(crate) order: usize,
}

impl Sample {}

/// A label id is just the number that represents the class
pub type LabelId = usize;



/// Lookup for a class index to a color
pub type Palette = Vec<Color>;


/// Fake data generator while we do not have the existing ones
fn generate_random_data(nb_samples: usize, nb_classes: usize) -> Vec<Sample> {
    //let mut rng = rand::thread_rng();

    let mut data = Vec::new();

    let mut current_label = 0;
    for i in 0..nb_samples {
        let current_order = i;
        if i % (nb_samples / nb_classes) == 0 && i != 0 {
            current_label += 1;
        }
        assert!(nb_classes > current_label);

        data.push(Sample {
            order: current_order,
            label: current_label,
        });
    }

    data
}

/// Generate an appropriate palette given the number of classes
/// Crash when more than 12 colors are expected
fn generate_palette(nb_classes: usize) -> Palette {
    [
        image::Rgb::<u8>([166, 206, 227]),
        image::Rgb::<u8>([31, 120, 180]),
        image::Rgb::<u8>([178, 223, 138]),
        image::Rgb::<u8>([51, 160, 44]),
        image::Rgb::<u8>([251, 154, 153]),
        image::Rgb::<u8>([227, 26, 28]),
        image::Rgb::<u8>([253, 191, 111]),
        image::Rgb::<u8>([255, 127, 0]),
        image::Rgb::<u8>([202, 178, 214]),
        image::Rgb::<u8>([106, 61, 154]),
        image::Rgb::<u8>([255, 255, 153]),
        image::Rgb::<u8>([177, 89, 40]),
    ][..nb_classes]
        .to_vec()
}

fn generate_relative_palette(nb_classes: usize) -> Palette {
    (0..nb_classes).into_iter()
        .map(|idx|{
            let amount = (idx as f64 * 256.0 / nb_classes as f64) as u8;
            image::Rgb::<u8>([amount, amount, amount])
        })
        .collect()
}

struct FractalImageBuilder {
    ordered_samples: Vec<(usize, Sample)>,
    width: usize,
    height: usize,
    background: Rgb<u8>
}

impl FractalImageBuilder {
    pub fn new(ordered_samples: Vec<(usize, Sample)>, width: usize, height: usize) -> Result<FractalImageBuilder, &'static str> {
        if width*height < ordered_samples.len() {
            Err("The image cannot embed the samples")
        }
        else{ 

            Self::new_with_background(ordered_samples, width, height, Rgb::<u8>([0,0,0]))
        }
    }

    pub fn new_with_background(
        ordered_samples: Vec<(usize, Sample)>,
        width: usize,
        height: usize,
        background: Rgb<u8>
    ) -> Result<FractalImageBuilder, &'static str> {
        if width * height < ordered_samples.len() {
            Err("The image cannot embed the samples")
        } else {
            Ok(FractalImageBuilder {
                ordered_samples,
                width,
                height,
                background: background
            })
        }
    }

    /// Generate a fake image in order to have an idea of the curve
    /// TODO reduce this copy paste stuff
    pub fn fake_image(width: usize, height: usize, nb_samples: usize) -> Image {
        let hole_size = width * height - nb_samples;
        let bits_required = hilbert::normalize::bits_required(nb_samples as u32 - 1);
        let _closest_power =
            2u32.pow(hilbert::normalize::smallest_power_of_two(nb_samples as u32) as u32);

        let first = palette::Hsv::from(palette::LinSrgb::new(1.0, 0.0, 0.0));
        let second = palette::Hsv::from(palette::LinSrgb::new(0.0, 1., 0.0));
        let gradient = palette::gradient::Gradient::new(vec![first, second]);

        let coordinates_and_color = (0..nb_samples)
            .into_iter()
            .map(|id| hilbert_axes(&(id + hole_size / 2).into(), bits_required, 2))
            .zip(gradient.take(nb_samples));

        use palette::encoding::pixel::Pixel;
        let mut img = Image::new(width as _, height as _);
        for (coordinate, color) in coordinates_and_color {
            assert_eq!(
                *img.get_pixel(coordinate[0], coordinate[1]),
                image::Rgb::<u8>([0, 0, 0])
            );
            let pixel: [u8; 3] = palette::LinSrgb::from(color).into_format().into_raw();
            img.put_pixel(coordinate[0], coordinate[1], image::Rgb(pixel));
        }

        img
    }

    /// Generate the coordinates to use within the image
    pub fn produce_coordinates(&self) -> impl Iterator<Item=(usize, Vec<u32>)> + '_ {
        let hole_size = self.width * self.height - self.nb_samples();
        let bits_required = hilbert::normalize::bits_required(self.nb_samples() as u32-1);
        let _closest_power = 2u32.pow(hilbert::normalize::smallest_power_of_two(self.nb_samples() as u32) as u32);

        self.ordered_samples
            .iter()
            .map(move |sample|{(
                sample.0,
                hilbert_axes(
                    &(sample.1.order + hole_size/2).into(), 
                    bits_required, 
                2)
            )})
        }

    pub fn nb_samples(&self) -> usize {
        self.ordered_samples.len()
    }

    /// Produce an image of the appropriate size
    pub fn produce_image(&self, palette: &[Color]) -> Image {
        let coordinates = self.produce_coordinates();

        let coordinates_and_color = self.ordered_samples.iter().zip(coordinates)
            .map(|( (_, ref sample), (_, ref coords))|{(
                coords.clone(),
                palette[sample.label])
            }).collect::<Vec<_>>();

        let mut img = Image::new(self.width as _, self.height as _);
        img.pixels_mut().for_each(|p| *p = self.background);
        
        for (coordinate, color) in coordinates_and_color.iter() {
            assert_eq!(
                *img.get_pixel(coordinate[0], coordinate[1]),
                self.background
            );
            img.put_pixel(coordinate[0], coordinate[1], *color);
        }


        let qual = quality::PixelNeighboorhoodQuality{
            kernel_size: 9
        };
        println!("Quality = {}", qual.compute(&img));

        img
    }
}

impl From<&reader::SampleRecord> for Sample {
    fn from(f: &reader::SampleRecord) -> Sample {
        Sample {
            label: f.groundtruth,
            order: f.order,
        }
    }
}


fn read_samples(path: &str) -> Vec<(usize, Sample)> {
    // Read the file
    let records = reader::read_file(&path);
    println!("Accuracy: {}%", accuracy::compute(&records));

    let mut samples : Vec<_> = records.iter()
    .enumerate()
    .map(|(idx, record)|{
        (idx, Sample::from(record))
    })
    .collect();

    // Order it appropriately
    samples.sort_by(|a, b|{
        a.1.order.cmp(&b.1.order)
    });



    samples
}


/// When real data is false, we do not care of the samples. Instead, we plot a gradient
pub fn to_fractal_pixels(path: &str, real_data: bool) {
    // Get the samples to draw
    let samples = read_samples(path);

    if real_data {

        // Generate the real one
        let nb_classes = samples.iter().unique_by(|s| s.1.label).count();
        let palette = generate_palette(nb_classes);
        let builder = FractalImageBuilder::new(samples.clone(), 128, 128).unwrap();
        let img = builder.produce_image(&palette);
        img.save(path.replace(".csv", ".png")).unwrap();


        // Generate the class highlighted one
        for i in 0..nb_classes {

            // build the modified palette
            let new_palette = {
                let mut p = Vec::new();
                p.resize(
                    nb_classes,
                    image::Rgb::<u8>([0, 0, 0])
                );
                p[i] = palette[i];
                p
            };

            FractalImageBuilder::new(samples.clone(), 128, 128).unwrap();
            let img = builder.produce_image(&new_palette);
            img.save(path.replace(".csv",  &format!("_class_{}.png", i))).unwrap();

        }
    } else {
        let nb_samples = samples.len();
        let img = FractalImageBuilder::fake_image(128, 128, nb_samples);
        img.save("fake.png");
    }
}

// There is a divergence when two consecutive labels have a different color
fn label_to_relative_divergence(data: &[(usize, Sample)]) -> Vec<(usize, Sample)> { 

    let mut diverged = Vec::new();
    for i in 0..data.len() {
        let mut count = 0;

        if i > 0 && data[i].1.label != data[i-1].1.label {
            count += 1;
        }

        if i < data.len()-1 && data[i].1.label != data[i+1].1.label {
            count += 1;
        }

        let mut clone = data[i].clone();
        clone.1.label = count;
        diverged.push(clone);
    }

    diverged
}

/// Instead of using one color per class, use one color per divergence
pub fn to_fractal_pixels_relative(path: &str) {
    // Get the samples to draw
    let samples = read_samples(path);

    let samples = label_to_relative_divergence(& samples);


    let nb_classes = samples.iter().unique_by(|s| s.1.label).count();
    let palette = generate_relative_palette(nb_classes);
    let builder = FractalImageBuilder::new_with_background(samples, 128, 128, Rgb::<u8>([173, 216, 230])).unwrap();
    let img = builder.produce_image(&palette);
    img.save(path.replace(".csv", "-relative.png")).unwrap();
}

fn image_to_relative(img: Image) -> Image {
    let mut copy = img.clone();
    let count_similar_pixels = |x: u32, y: u32| {
        let mut count = 4;
        for (dx, dy) in [(-1, 0), (1, 0), (0, -1), (0, 1)].into_iter() {
            let real_x = x as i64 + dx;
            let real_y = y as i64 + dy;

            if real_x < 0 || real_y < 0 {
                continue; //(impossible to test);
            }
            
            let real_x = real_x as u32;
            let real_y = real_y as u32;

            if real_x >= img.width() || real_y >= img.height() {
                continue; // impossible to test
            }

            if *img.get_pixel(real_x, real_x) != *img.get_pixel(x, y) {
                count = count - 1;
            }
            

        }

        // Return the number of pixels equal
        count
    };

    for x in 0..img.width() {
        for y in 0..img.height() {
            let count = count_similar_pixels(x, y);
            let value = (count as f64 / 4.0 * 256.0) as u8;
            *copy.get_pixel_mut(x, y) = image::Rgb::<u8>([value, value, value]);
        }
    }

    copy
}

// Yet a relative version, but difference is based on the pixel representation
pub fn to_fractal_pixels_relative2(path: &str) {
    // Get the samples to draw
    let samples = read_samples(path);
//    let samples = read_samples(path).iter().map(|t| t.1).collect::<Vec<Sample>>();



    let nb_classes = samples.iter().unique_by(|s| s.1.label).count();
    let palette = generate_relative_palette(nb_classes);
    let builder = FractalImageBuilder::new(samples, 128, 128).unwrap();
    let img = builder.produce_image(&palette);
    let img = image_to_relative(img);

    img.save(path.replace(".csv", "-relative2.png")).unwrap();

}


#[derive(serde::Serialize)]
struct Position<T> {
    x: T,
    y: T,
}

impl<T> Position<T> {
    pub fn new(x: T, y: T) -> Self {
        Position { x, y }
    }
}

pub fn to_fractal_points(path: &str, out: &str) -> std::io::Result<()> {
    eprintln!("TODO reorder in the same order than the input file or store the class/id");
    let samples = read_samples(path);
    let builder = FractalImageBuilder::new(samples, 128, 128).unwrap();
    let positions = {
        let mut data = builder.produce_coordinates()
                            .map(|(idx, c)| (idx, Position::new(c[0], c[1])))
                            .collect::<Vec<(usize, Position<u32>)>>();
        data.sort_by_key(|&(idx, _)| idx.clone());
        data
    };
    let mut wtr = csv::Writer::from_writer(std::io::BufWriter::new(std::fs::File::create(out)?));
    //wtr.write_record(&["x", "y"])?;
    positions.iter().for_each(|(idx, p)| wtr.serialize(&p).unwrap()); // TODO properly handle error
    wtr.flush()?;
    Ok(())
}

pub fn to_barycenter_points(path: &str, out: &str) -> std::io::Result<()> {
    eprintln!("TODO reorder in the same order than the input file or store the class/id");

    let samples = read_samples(path);
    let builder = FractalImageBuilder::new(samples, 128, 128).unwrap();
    let positions = builder.produce_coordinates()
                            .map(|(idx, c)| barycenter::Point::new(c[0] as f64, c[1] as f64))
                            .collect::<Vec<_>>();
    let mut bary = barycenter::BarycenterLayout::new(positions);
    let positions = bary.run();
    let mut wtr = csv::Writer::from_writer(std::io::BufWriter::new(std::fs::File::create(out)?));
    //wtr.write_record(&["x", "y"])?;
    positions
        .iter()
        .for_each(|p| wtr.serialize(&Position::new(p.x, p.y)).unwrap()); // TODO properly handle error
    wtr.flush()?;
    Ok(())
}



fn main() {
    //   test();

    let args = clap::App::new("Ordered and labelled samples visualization")
        .author("Romain Giot <romain.giot@u-bordeaux.fr>")
        .version("0.2.0")
        .arg(
            clap::Arg::with_name("CURVE")
                .long("curve")
                .possible_values(&["hilbert"])
                .default_value("hilbert")
                .help(
                    "Specify which fractal curve to use. Only Hilbert is supported at the moment",
                ),
        )
        .arg(
            clap::Arg::with_name("INPUT")
                .long("input")
                .help("File name to read")
                .required(true)
                .takes_value(true),
        )
        .arg(
            clap::Arg::with_name("KIND")
                .long("kind")
                .possible_values(&[
                    "fractal_pixels",
                    "fractal_pixels_relative",
                    "fractal_pixels_relative2",
                    "fractal_pixels_ordering",
                    "fractal_points",
                    "barycenter_points",
                ])
                .default_value("fractal_pixels"),
        )
        .get_matches();

    let input = args.value_of("INPUT").unwrap();
    match args.value_of("KIND").unwrap() {
        "fractal_pixels" => {
            to_fractal_pixels(input, true);
        }
        "fractal_pixels_ordering" => {
            to_fractal_pixels(input, false);
        }
        "fractal_pixels_relative" => {
            to_fractal_pixels_relative(input);
        }
        "fractal_pixels_relative2" => {
            to_fractal_pixels_relative2(input);
        }
        "fractal_points" => {
            to_fractal_points(input, &input.replace(".csv", ".xy")).unwrap();
        }
        "barycenter_points" => {
            to_barycenter_points(input, &input.replace(".csv", ".xybary")).unwrap()
        }
        _ => unreachable!(),
    }
}
