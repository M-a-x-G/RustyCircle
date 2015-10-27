/*

Copyright 2015 Max Gregor

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.

*/

extern crate imagefmt;
extern crate docopt;
extern crate rustc_serialize;

use docopt::Docopt;
use imagefmt::{ColFmt, ColType};

/// Main function called on excecution

fn main() {

	// usage string for comandline help
	const USAGE: &'static str = "
Usage:
 rusty_circle <center-x> <center-y> <radius>
";

	// Struct for comandline values
	#[derive(RustcDecodable)]
	struct Args {
		arg_center_x: String,
		arg_center_y: String,
		arg_radius: String,
	}

	// Parse argv and exit the program with an error message if it fails.
	let args: Args = Docopt::new(USAGE)
	.and_then(|d| d.decode())
	.unwrap_or_else(|e| e.exit());

	let redius_str = args.arg_radius;
	let cx_str = args.arg_center_x;
	let cy_str = args.arg_center_y;
	// center x coordinate of the circle
	let cx: i32;
	// center y coordinate of the circle
	let cy: i32;
	// center radius of the circle
	let radius: i32;
	// height in pixel of the outputimage
	let h = 901;
	// width in pixel of the outputimage
	let w = 1201;
	// vector for image generation where one pixel is represented by three following entries (Red, Green, Blue).
	let mut image_vector: Vec<u8> = vec![255;(3 * w * h)];
	let w3: i32 = (w * 3) as i32;

	// parse the center x coordinate
	cx = match cx_str.trim().parse(){
						Ok(num) => if num > 0 && num <= 200 {num} else {println!("Please type a x coordinate between 1-200!"); -1},
						Err (_) => {println!("Please type x y coordinate between 1-200!"); -1},};

	// parse the center y coordinate
	cy = match cy_str.trim().parse(){
			Ok(num) => if num > 0 && num <= 150 {num} else {println!("Please type a y coordinate between 1-150!"); -1},
			Err (_) => {println!("Please type a y coordinate between 1-150!"); -1},};

	// parse the radius
	radius = match redius_str.trim().parse(){
			Ok(num) => if num > 0 && num <= 100{num} else {println!("Please type a radius between 1-100!"); -1},
			Err (_) => {println!("Please type a radius between 1-100!"); -1},};

	if cx != -1 && cy != -1 && radius != -1 {

		prepare_grid(w3, &mut image_vector);
		prepare_circle(cx, cy, radius, w3, &mut image_vector);

		//  write image
		imagefmt::Image{w: w, h: h, fmt:ColFmt::RGB, buf:image_vector}.write("circle.png", ColType::Color).unwrap();

		println!("Circle image succussfully saved to circle.png");

	}


}

/// Sets points for a grid in a `Vec<u8>` for image generation where one pixel is represented by three following entries (Red, Green, Blue).
/// Each line of the grid is one pixel wide. The Grid has a widh of 200 points and a heigth of 150 points where one point is 5px x 5px.
/// Additionaly the value for three times the overall width of the output image (`w3`) is needed.

fn prepare_grid(w3: i32, image_vector: &mut Vec<u8>){
	let mut line_counter = 0;
	for i in 0..image_vector.len() {
		if i % (w3 as usize) == 0 {
			for j in 0..201{
				let new_i = (j * 6 * 3) + i;
				image_vector[new_i] = 0;
				image_vector[new_i+1] = 0;
				image_vector[new_i+2] = 0;
			}
			if line_counter == 0{
				for j in 0..w3 as usize{
					image_vector[(j + i) as usize] = 0;
				}
			}

			line_counter += 1;
			if line_counter == 6 {
				line_counter = 0;
			}
		}
	}
}

/// Sets points for a circle with given x (`cx`), y (`cy`) as center coordinates and a radius (`radius`) in a `Vec<u8>` for image generation where
/// one pixel is represented by three following entries (Red, Green, Blue).
/// Additionaly the value for three times the overall width of the output image (`w3`) is needed.

fn prepare_circle(cx: i32, cy: i32, radius: i32, w3: i32, image_vector: &mut Vec<u8>) {
	let mut f: i32 = 1 - radius as i32;
	let mut dd_f_x: i32 = 0;
	let mut dd_f_y: i32 = -2 * radius as i32;
	let mut x: i32 = 0;
	let mut y: i32 = radius;

	// Set right, left, top and bottom point
	set_point(cx, cy + radius, image_vector, w3);
	set_point(cx, cy - radius, image_vector, w3);
	set_point(cx + radius, cy, image_vector, w3);
	set_point(cx - radius, cy, image_vector, w3);

	while x < y {
		if f >= 0 {
			y -= 1;
			dd_f_y += 2;
			f += dd_f_y;
		}
		x += 1;
		dd_f_x += 2;
		f += dd_f_x + 1;

		set_point(cx + x, cy + y, image_vector, w3);
		set_point(cx - x, cy + y, image_vector, w3);
		set_point(cx + x, cy - y, image_vector, w3);
		set_point(cx - x, cy - y, image_vector, w3);
		set_point(cx + y, cy + x, image_vector, w3);
		set_point(cx - y, cy + x, image_vector, w3);
		set_point(cx + y, cy - x, image_vector, w3);
		set_point(cx - y, cy - x, image_vector, w3);
	}
}

/// Sets a point to black (0, 0, 0) with given `x` and `y` coordinates in a `Vec<u8>` for image generation where
/// one pixel is represented by three following entries (Red, Green, Blue).
/// Additionaly the value for three times the overall width of the output image (`w3`) is needed.

fn set_point(x: i32, y: i32, image_vector: &mut Vec<u8>, w3: i32){
	if x<=200 && y<=150 && x > 0 && y > 0  {
		let x_2d = x * 3 + (x - 1) * 15 - 1;
		let y_2d = y + (y - 1)* 5;
		let offset = y_2d * w3;

		for i in 0..5 {
			for j in 1..16 {
				image_vector[(offset + x_2d + j + (i * w3)) as usize] = 0;
			}
		}
	}
}
