use num_complex::Complex;
use rayon::prelude::*;
use image::ImageBuffer;
use std::path;

fn main() {
    let pallets: Vec<Vec<image::Rgb<u8>>> = vec![
        // Original
        vec![
            image::Rgb::<u8>([0, 7, 100]),
            image::Rgb::<u8>([32, 107, 203]),
            image::Rgb::<u8>([237, 255, 255]),
            image::Rgb::<u8>([255, 170, 0]),
            image::Rgb::<u8>([0, 2, 0]),
            image::Rgb::<u8>([0, 7, 100]),
        ],
        // Fire
        vec![
            image::Rgb::<u8>([20, 0, 0]),
            image::Rgb::<u8>([255, 20, 0]),
            image::Rgb::<u8>([255, 200, 0]),
            image::Rgb::<u8>([255, 20, 0]),
            image::Rgb::<u8>([20, 0, 0]),
        ],
        // Electric
        vec![
            image::Rgb::<u8>([0, 0, 0]),
            image::Rgb::<u8>([0, 0, 200]),
            image::Rgb::<u8>([255, 255, 255]),
            image::Rgb::<u8>([0, 0, 200]),
            image::Rgb::<u8>([0, 0, 0]),
        ],
        // Gold
        vec![
            image::Rgb::<u8>([85, 47, 0]),
            image::Rgb::<u8>([255, 171, 12]),
            image::Rgb::<u8>([255, 247, 127]),
            image::Rgb::<u8>([255, 171, 12]),
            image::Rgb::<u8>([85, 47, 0]),
        ],
        // RGB gradient
        vec![
            image::Rgb::<u8>([255, 0, 0]),
            image::Rgb::<u8>([255, 255, 0]),
            image::Rgb::<u8>([0, 255, 0]),
            image::Rgb::<u8>([0, 255, 255]),
            image::Rgb::<u8>([0, 0, 255]),
            image::Rgb::<u8>([255, 0, 255]),
            image::Rgb::<u8>([255, 0, 0]),
        ],
        // RGB
        vec![
            image::Rgb::<u8>([255, 0, 0]),
            image::Rgb::<u8>([255, 255, 0]),
            image::Rgb::<u8>([0, 255, 0]),
            image::Rgb::<u8>([0, 255, 255]),
            image::Rgb::<u8>([0, 0, 255]),
            image::Rgb::<u8>([255, 0, 255]),
            image::Rgb::<u8>([255, 0, 0]),
        ],
        // Black and white gradient
        vec![
            image::Rgb::<u8>([0, 0, 0]),
            image::Rgb::<u8>([255, 255, 255]),
            image::Rgb::<u8>([0, 0, 0]),
        ],
        // Black or white
        vec![
            image::Rgb::<u8>([0, 0, 0]),
            image::Rgb::<u8>([255, 255, 255]),
            image::Rgb::<u8>([0, 0, 0]),
        ],
        // Set only
        vec![
            image::Rgb::<u8>([255, 255, 255]),
            image::Rgb::<u8>([255, 255, 255]),
        ],
    ];



    let height = 1000;
    let width = 1000;

    let xa = -2.0;
    let xb = 1.0;
    let ya = -1.5;
    let yb = 1.5;
    let max_it = 256;

    let data = (0..width * height).into_par_iter().map(|offset| {
        let img_x = offset % width;
        let img_y = offset / width;

        let x = (img_x as f64) * (xb - xa) / (width as f64 - 1.0f64) + xa;
        let y = (img_y as f64) * (yb - ya) / (height as f64 - 1.0f64) + ya;

        return compute_iter(x, y, max_it);
    }).collect::<Vec<usize>>();

    let mut img = ImageBuffer::new(width as u32, height as u32);

    for (x, y, pixel) in img.enumerate_pixels_mut() {

        let offset = (y * width as u32 + x) as usize;
        let it = data[offset];

        let color = get_color(&pallets[1], (it as f64) / (max_it as f64));

        *pixel = color;
    }

    img.save(path::Path::new("fractal.png")).unwrap();

}


fn get_color(pallet: &Vec<image::Rgb<u8>>, value: f64) -> image::Rgb<u8> {    
    for i in 0..pallet.len() - 1 {
        let min = (i as f64) / (pallet.len() as f64);
        let max = ((i + 1) as f64) / (pallet.len() as f64);

        if value >= min && value <= max {
            let v = (value - min) / (max - min);
            // use rgb palette (palette[4])

            return image::Rgb::<u8>([
                (pallet[i].0[0] as f64 * (1.0 - v) + pallet[i + 1].0[0] as f64 * v) as u8,
                (pallet[i].0[1] as f64 * (1.0 - v) + pallet[i + 1].0[1] as f64 * v) as u8,
                (pallet[i].0[2] as f64 * (1.0 - v) + pallet[i + 1].0[2] as f64 * v) as u8,
            ]);
        }
    }

    return image::Rgb([0, 0, 0]);
}

/*

float4image::Rgb([float] ite as u8rations, fl as u8oat max_ite as u8rat as u8]ions, __global float4* pallet,image::Rgbs_nb){
	fl as u8oat value = as u8 iterations as u8 /  as u8max_iterations;
	flimage::Rgb = (float4 as u8)(1, 1, 1,  as u81);
	float  as u8min as u8_value;
	float max_value;
	for (int i = 0; i < image::Rgbs_nb; i++) as u8{
		min_val as u8ue = (float as u8ima as u8ge::Rgbs_nb;
		ma as u8x_value = ( as u8float)(i +  as u8ima as u8ge::Rgbs_nb;
		if as u8 (value >=  as u8min_value & as u8& v as u8alue <= max_value)image::Rgb = mix(pal as u8let[i], pal as u8let[i + 1], as u8 (v as u8alue - min_valuimage::Rgbs_nb);
			 as u8break;
		}
 as u8	}
	reimage as u8::R as u8gb;
}
*/

fn compute_iter(x: f64, y: f64, max_it: usize) -> usize {
    let c = Complex::new(x, y);
    let mut z = Complex::new(0f64, 0f64);


    let mut i = 0;
    while i < max_it && z.norm() < 4.0 {
        z = z * z + c;
        i+=1;
    }

    return i;
}