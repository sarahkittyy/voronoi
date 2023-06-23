use colored::*;
use image::{DynamicImage, Rgb, Rgb32FImage};
use rand::prelude::*;
use std::env;
use std::process::exit;
use std::str::FromStr;

#[derive(Clone, Copy, PartialEq, Debug)]
struct Vec2<T> {
    x: T,
    y: T,
}

impl<T> Vec2<T> {
    fn new(x: T, y: T) -> Self {
        Vec2 { x, y }
    }
}
impl Vec2<f32> {
    fn dist(&self, other: Vec2<f32>) -> f32 {
        ((other.x - self.x).powf(2.0) + (other.y - self.y).powf(2.0)).sqrt()
    }
}
impl<T: FromStr> FromStr for Vec2<T> {
    type Err = <T as FromStr>::Err;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let arr = s.split(',').collect::<Vec<&str>>();
        let x = T::from_str(
            &arr.get(0)
                .expect("Size requires two comma-separated positive integers."),
        )?;
        let y = T::from_str(
            &arr.get(1)
                .expect("Size requires two comma-separated positive integers."),
        )?;
        Ok(Vec2::new(x, y))
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}

impl Color {
    fn rand() -> Self {
        Color {
            r: random(),
            g: random(),
            b: random(),
        }
    }
}

impl From<Color> for Rgb<f32> {
    fn from(value: Color) -> Self {
        Rgb::<f32>([value.r, value.g, value.b])
    }
}

struct Seed {
    pos: Vec2<f32>,
    color: Color,
}

struct Voronoi {
    seeds: Vec<Seed>,
}

impl Voronoi {
    fn closest_seed(&self, pos: Vec2<f32>) -> &Seed {
        self.seeds
            .iter()
            .reduce(|min, seed| {
                let cdist = seed.pos.dist(pos);
                let mindist = min.pos.dist(pos);
                if cdist < mindist {
                    seed
                } else {
                    min
                }
            })
            .expect("Seeds is empty.")
    }
    fn to_image(&self, dim: Vec2<u32>) -> Rgb32FImage {
        let mut image = Rgb32FImage::new(dim.x, dim.y);

        for x in 0..dim.x {
            for y in 0..dim.y {
                let tx = x as f32 / dim.x as f32;
                let ty = y as f32 / dim.y as f32;
                let closest_seed = self.closest_seed(Vec2::new(tx, ty));
                image.put_pixel(x, y, closest_seed.color.into());
            }
        }

        image
    }
}

#[derive(Debug)]
struct Config {
    seed_count: u32,
    output: String,
    dim: Vec2<u32>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            seed_count: 10,
            output: "voronoi.png".to_owned(),
            dim: Vec2::new(256, 256),
        }
    }
}

impl Config {
    fn parse_args(args: Vec<String>) -> Self {
        let app_name = &args[0];
        let args = &args[1..];
        // help menu
        if args.contains(&"-h".to_owned()) || args.contains(&"--help".to_owned()) {
            println!("{} {} [Options] [filename]", "Usage:".cyan(), app_name);
            println!("Options:");
            println!("\t--size | -s <x,y>\tImage output size (default 256x256");
            println!("\t--count | -c N\t\tSeed count (default 10)");
            println!("\tfilename\t\tOutput path (default voronoi.png)");

            println!(
                "Example: {} {}\tOutputs a 256x256 image as out.png with 20 seeds.",
                app_name.cyan(),
                "-s 256,256 -c 20 out.png".cyan()
            );
            exit(0);
        } else {
            let seed_count: u32 = args
                .iter()
                .skip_while(|&s| s != "-c" && s != "--count")
                .nth(1)
                .map_or(10, |v| {
                    u32::from_str(v).expect("Invalid parameter for --count.")
                });
            let dim: Vec2<u32> = args
                .iter()
                .skip_while(|&s| s != "-s" && s != "--size")
                .nth(1)
                .map_or(Vec2::new(256, 256), |v| {
                    Vec2::<u32>::from_str(v).expect("Invalid parameter for --size.")
                });
            let output: String = args.last().unwrap_or(&"voronoi.png".to_owned()).clone();
            Config {
                seed_count,
                dim,
                output,
                ..Default::default()
            }
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let config = Config::parse_args(args);

    let game = Voronoi {
        seeds: (0..config.seed_count)
            .map(|_| Seed {
                pos: Vec2::<f32>::new(random(), random()),
                color: Color::rand(),
            })
            .collect(),
    };

    DynamicImage::ImageRgb32F(game.to_image(config.dim))
        .to_rgb8()
        .save(&config.output)
        .expect("Could not save output image.");

    println!(
        "{}",
        format!(
            "Wrote {}-seed voronoi to {}x{} {}",
            config.seed_count, config.dim.x, config.dim.y, config.output
        )
        .green()
    );
}
