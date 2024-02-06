extern crate image;

use image::{GenericImageView, DynamicImage, Rgba};
use std::time::Instant;

fn read_image_pixels(image_path: &str) -> Vec<Vec<Rgba<u8>>>{
    // Open the image file
    let mut matrix: Vec<Vec<Rgba<u8>>> = vec![];

    if let Ok(img) = image::open(image_path) {
        // Get dimensions of the image
        let (width, height) = img.dimensions();

        for y in 0..height {
            let mut line: Vec<Rgba<u8>> = vec![];
            for x in 0..width {

                line.push(img.get_pixel(x, y));
            }
            matrix.push(line);
        }
    } else {
        println!("Failed to open the image.");
    }
    return matrix;
}

fn create_image_from_matrix(matrix: &Vec<Vec<Rgba<u8>>>) -> DynamicImage {
    let width = matrix[0].len() as u32;
    let height = matrix.len() as u32;

    let mut img_buf = Vec::new();

    // Flatten the matrix into a vector of pixels
    for row in matrix {
        img_buf.extend(row.into_iter().flat_map(|pixel| pixel.0));
    }

    // Create a DynamicImage from the vector of pixels
    DynamicImage::ImageRgba8(image::ImageBuffer::from_vec(width, height, img_buf).expect("Failed to create image"))
}

fn convert_i32_to_u8(value: i32) -> u8 {
    value.clamp(0, 255) as u8
}

fn apply_kernel(neighborhood: &[Vec<Rgba<u8>>], kernel: &[Vec<i32>]) -> Rgba<u8> {
    let mut result_pixel: Rgba<u8> = Rgba([0, 0, 0, 255]);

    
    for i in 0..kernel.len() {
        for j in 0..kernel[i].len() {
            result_pixel[0] += convert_i32_to_u8(neighborhood[i][j][0] as i32 * kernel[i][j]);
            result_pixel[1] += convert_i32_to_u8(neighborhood[i][j][1] as i32 * kernel[i][j]);
            result_pixel[2] += convert_i32_to_u8(neighborhood[i][j][2] as i32 * kernel[i][j]);
            
        }
    }
    return result_pixel;
}



fn process_image(matrix: &Vec<Vec<Rgba<u8>>>, kernel: &Vec<Vec<i32>>) -> Vec<Vec<Rgba<u8>>> {
    let largeur: usize = matrix.len();
    let hauteur: usize = matrix[0].len();
    let kernel_largeur: usize = kernel.len();
    let kernel_hauteur: usize = kernel[0].len();

    let mut result_matrix: Vec<Vec<Rgba<u8>>> = vec![];

    for i in 0..largeur {
        let mut row = vec![];

        for j in 0..hauteur {
            // Getting the neighborhood
            let mut neighborhood: Vec<Vec<Rgba<u8>>> = vec![];
            for k in 0..kernel_largeur {
                let mut pixel_line: Vec<Rgba<u8>> = vec![];

                for l in 0..kernel_hauteur {
                    let pos_i = if (0 <= i as isize - 1 + l as isize) && ((i as isize) - 1 + (l as isize) < (largeur as isize)) {
                        (i as isize - 1 + l as isize) as usize
                    } else {
                        i
                    };

                    let pos_j = if (0 <= j as isize - 1 + k as isize) && ((j as isize) - 1 + (k as isize) < (hauteur as isize)) {
                        (j as isize - 1 + k as isize) as usize
                    } else {
                        j
                    };
                    pixel_line.push(matrix[pos_i][pos_j]);
                }
                neighborhood.push(pixel_line);
            }
            row.push(apply_kernel(&neighborhood, &kernel));
        }

        result_matrix.push(row);
    }

    result_matrix
}





fn main() {
    let mut kernel1: Vec<Vec<i32>> = vec![];
    kernel1.push(vec![0, -1, 0]);
    kernel1.push(vec![-1, 4, -1]);
    kernel1.push(vec![0, -1, 0]);

    let mut kernel2: Vec<Vec<i32>> = vec![];
    kernel2.push(vec![0, 0, 0]);
    kernel2.push(vec![0, 1, 0]);
    kernel2.push(vec![0, 0, 0]);

    let mut img_paths: Vec<&str> = vec![];
    img_paths.push("./img/50x50.png");
    img_paths.push("./img/100x100.png");
    img_paths.push("./img/200x200.png");
    img_paths.push("./img/300x300.png");
    img_paths.push("./img/400x400.png");
    img_paths.push("./img/1000x1000.png");
    img_paths.push("./img/1500x1500.png");
    img_paths.push("./img/2000x2000.png");

    for image_path in img_paths{
        let t0: Instant = Instant::now();
        let matrix: Vec<Vec<Rgba<u8>>> = read_image_pixels(image_path);
        let matrix1: Vec<Vec<Rgba<u8>>> = process_image(&matrix, &kernel1);
        create_image_from_matrix(&matrix1);
        let t1: Instant = Instant::now();
        let elapsed_time: std::time::Duration = t1 - t0;
        print!("Image : {}, Time : {}\n", image_path, elapsed_time.as_secs_f64());
    }
    

    

}
