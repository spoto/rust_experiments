use std::env;
use std::fs::File;
use std::str::FromStr;
use image::ColorType;
use image::png::PNGEncoder;
use num::Complex;

fn main() {
    parallel_crossbeam();
}

struct Window {
    bounds: (usize, usize),
    upper_left: Complex<f64>,
    lower_right: Complex<f64>
}

impl Window {
    fn pixel_to_point(&self, pixel: (usize, usize)) -> Complex<f64> {
        let (width, height) = (self.lower_right.re - self.upper_left.re, self.upper_left.im - self.lower_right.im);
        Complex {
            re: self.upper_left.re + pixel.0 as f64 * width / self.bounds.0 as f64,
            im: self.upper_left.im - pixel.1 as f64 * height / self.bounds.1 as f64
        }
    }
}

#[test]
fn test_pixel_to_point() {
    let window = Window {
        bounds: (100, 200),
        upper_left: Complex { re: -1.0, im: 1.0},
        lower_right: Complex { re: 1.0, im: -1.0}
    };

    assert_eq!(window.pixel_to_point((25, 175)), Complex { re: -0.5, im: -0.75 });
}

fn sequential() {
    let args: Vec<String> = env::args().collect();
    check(&args);
    let pars : Window = extract_window(&args);
    let mut pixels = vec![0; pars.bounds.0 * pars.bounds.1];
    render(&mut pixels, &pars);
    write_image(&args[1], &pixels, pars.bounds)
        .expect("error writing PNG file");
}

fn parallel_crossbeam() {
    let args: Vec<String> = env::args().collect();
    check(&args);
    let window= extract_window(&args);
    let mut pixels = vec![0; window.bounds.0 * window.bounds.1];
    render_parallel_crossbeam(&mut pixels, &window);
    write_image(&args[1], &pixels, window.bounds)
        .expect("error writing PNG file");
}

fn render(pixels: &mut [u8], window: &Window) {
    assert_eq!(pixels.len(), window.bounds.0 * window.bounds.1);

    for row in 0..window.bounds.1 {
        for column in 0..window.bounds.0 {
            let point = window.pixel_to_point((column, row));
            pixels[row * window.bounds.0 + column] =
                match escape_time(point, 255) {
                    None => 0,
                    Some(count) => 255 - count as u8
                }
        }
    }
}

fn render_parallel_crossbeam(pixels: &mut Vec<u8>, window: &Window) {
    let threads = 8;
    let rows_per_band = window.bounds.1 / threads + 1;
    let bands: Vec<&mut [u8]> = pixels.chunks_mut(rows_per_band * window.bounds.0).collect();
    crossbeam::scope(|spawner| {
        for (i, band) in bands.into_iter().enumerate() {
            let top = rows_per_band * i;
            let height = band.len() / window.bounds.0;
            let subwindow = Window {
                bounds: (window.bounds.0, height),
                upper_left: window.pixel_to_point((0, top)),
                lower_right: window.pixel_to_point((window.bounds.0, top + height))
            };
            spawner.spawn(move |_| {
                render(band, &subwindow);
            });
        }
    }).unwrap();
}

fn write_image(filename: &str, pixels: &[u8], bounds: (usize, usize)) -> Result<(), std::io::Error> {
    let output = File::create(filename)?;
    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels, bounds.0 as u32, bounds.1 as u32, ColorType::Gray(8))?;
    Ok(())
}

fn extract_window(args: &Vec<String>) -> Window {
    Window {
        bounds: extract_bounds(args),
        upper_left: parse_upper_left(args),
        lower_right: parse_lower_right(args)
    }
}

fn parse_upper_left(args: &Vec<String>) -> Complex<f64> {
    parse_complex(&args[3])
        .expect("error parsing upper left corner point")
}

fn parse_lower_right(args: &Vec<String>) -> Complex<f64> {
    parse_complex(&args[4])
        .expect("error parsing lower right corner point")
}

fn extract_bounds(args: &Vec<String>) -> (usize,usize) {
    parse_pair(&args[2], 'x')
        .expect("error parsing image dimensions")
}

fn check(args: &Vec<String>) {
    if args.len() != 5 {
        eprintln!("Usage: {} FILE PIXELS UPPERLEFT LOWERRIGHT", args[0]);
        eprintln!("Example: {} mandel.png 1000x750 -1.20,0.35 -1,0.20", args[0]);
        std::process::exit(1);
    }
}

fn escape_time(c: Complex<f64>, limit: usize) -> Option<usize> {
    let mut z = Complex {re: 0.0, im: 0. };
    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }

    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    match s.find(separator) {
        None => None,
        Some(index) =>
            match (T::from_str(&s[..index]), T::from_str(&s[index + 1..])) {
                (Ok(l), Ok(r)) => Some((l, r)),
                _ => None
            }
    }
}

#[test]
fn test_parse_pair() {
    assert_eq!(parse_pair::<i32>("",    ','), None);
    assert_eq!(parse_pair::<i32>("10,",    ','), None);
    assert_eq!(parse_pair::<i32>(",10",    ','), None);
    assert_eq!(parse_pair::<i32>("10,20",    ','), Some((10, 20)));
    assert_eq!(parse_pair::<i32>("10,20xy",    ','), None);
    assert_eq!(parse_pair::<f64>("0.5x",    'x'), None);
    assert_eq!(parse_pair::<f64>("0.5x1.5",    'x'), Some((0.5, 1.5)));
}

fn parse_complex(s: &str) -> Option<Complex<f64>> {
    match parse_pair(s, ',') {
        Some((re, im)) => Some(Complex {re, im}),
        None => None
    }
}

#[test]
fn test_parse_complex() {
    assert_eq!(parse_complex("1.25,-0.0625"), Some(Complex {re: 1.25, im: -0.0625}));
    assert_eq!(parse_complex(",-0.0625"), None);
}