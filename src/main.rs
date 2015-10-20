extern crate imagefmt;

use docopt::Docopt;
use std::io;
use std::cmp::Ordering;
use imagefmt::{ColFmt, ColType};

fn main() {

	println!("
 8888b. db    db .d8888. d888888b db    db
88  `8D 88    88 88'  YP `  88  ' `8b  d8'
88oobY' 88    88 `8bo.      88     `8bd8'
88`8b   88    88   `Y8b.    88       88
88 `88. 88b  d88 db   8D    88       88
88   YD ~Y8888P' `8888Y'    YP       YP


 .o88b. d888888b d8888b.  .o88b. db      d88888b
d8P  Y8   `88'   88  `8D d8P  Y8 88      88'
8P         88    88oobY' 8P      88      88ooooo
8b         88    88`8b   8b      88      88
Y8b  d8   .88.   88 `88. Y8b  d8 88booo. 88.
 `Y88P' Y888888P 88   YD  `Y88P' Y88888P Y88888P\n\n");

	let mut radius = "";
	let mut cx = "";
	let mut cy = "";
	let cxnum: i32 = 200;
	let cynum: i32 = 150;
	let radiusnum: i32 = 20;

//	loop {
//			cx = String::new();
//		println!("\nPlease enter the center x coordinate: ");
//		match io::stdin().read_line(&mut cx) {
//				Ok(line) => line,
//				Err(_) => {println!("failed to read x coordinate"); continue},
//			};
//		println!("Inhalt {}", cx);
//		cxnum = match cx.trim().parse(){
//				Ok(num) => if num > 0 && num <= 150 {num} else {println!("Please type a number between 1-150!"); continue},
//				Err (err) => {println!("Please type a number between 1-150 ({})!", err); continue},
//			};
//		break;
//	}
//	loop {
//		cy = String::new();
//		println!("\nPlease ender the center y coordinate: ");
//		match io::stdin().read_line(&mut cy) {
//				Ok(line) => line,
//				Err(_) => {println!("failed to read y coordinate"); continue},
//			};
//
//		cynum = match cx.trim().parse(){
//				Ok(num) => if num > 0 && num <= 200 {num} else {println!("Please type a number between 1-200!"); continue},
//				Err (_) => {println!("Please type a number between 1-200!"); continue},
//			};
//		break;
//	}
//	loop {
//		radius = String::new();
//		println!("\nPlease enter5 the radius: ");
//		match io::stdin().read_line(&mut radius){
//				Ok(line) => line,
//				Err(_) => {println!("failed to read radius"); continue},
//			};
//
//		radiusnum = match radius.trim().parse(){
//				Ok(num) => if num > 0 && num < 100{num} else {println!("Please type a number between 0-99!"); continue},
//				Err (_) => {println!("Please type a number between 0-99"); continue},
//			};
//		break;
//	}


	let h = 901;
	let w = 1201;
	let mut image_vector: Vec<u8> = vec![255;(3 * w * h)];

	let mut xc = 0;
	let mut yc = 0;
	let w3: i32 = (w * 3) as i32;

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
					xc += 1;

					//					if yc == cynum && xc == cxnum {
					//						for k in 1..5 {
					//							let shift_factor = k * (w3) + i + j;
					//							for l in 0..16 {
					//								image_vector[ shift_factor + l] = 0;
					//							}
					//						}
					//					}
				}
				xc = 0
			}
			line_counter += 1;

			if line_counter == 6 {
				line_counter = 0;
			}
			yc += 1;
		}
	}

	let mut f: i32 = (1 - radiusnum as i32);
	let mut ddF_x: i32 = 0;
	let mut ddF_y: i32 = (-2 * radiusnum as i32);
	let mut x: i32 = 0;
	let mut y: i32 = radiusnum;

	// Draw right, left, top and bottom point
//	println!("{} : {}", x0, (cynum + radiusnum));
//	println!("{} : {}", x0, (cynum - radiusnum));
//	println!("{} : {}", (x0 +radiusnum), cynum);
//	println!("{} : {}", (x0 - radiusnum), cynum);

	setPoint(cxnum, cynum + radiusnum, &mut image_vector, w3);
	setPoint(cxnum, cynum - radiusnum, &mut image_vector, w3);
	setPoint(cxnum + radiusnum, cynum, &mut image_vector, w3);
	setPoint(cxnum - radiusnum, cynum, &mut image_vector, w3);

	while x < y {
		if f >= 0 {
			y -= 1;
			ddF_y += 2;
			f += ddF_y;
		}
		x += 1;
		ddF_x += 2;
		f += ddF_x + 1;

//		println!("{} : {}", (cxnum + x), (cynum + y));
//		println!("{} : {}", (cxnum - x), (cynum + y));
//		println!("{} : {}", (cxnum + x), (cynum - y));
//		println!("{} : {}", (cxnum - x), (cynum - y));
//		println!("{} : {}", (cxnum + y), (cynum + x));
//		println!("{} : {}", (cxnum - y), (cynum + x));
//		println!("{} : {}", (cxnum + y), (cynum - x));
//		println!("{} : {}", (cxnum - y), (cynum - x));

		setPoint(cxnum + x, cynum + y, &mut image_vector, w3);
		setPoint(cxnum - x, cynum + y, &mut image_vector, w3);
		setPoint(cxnum + x, cynum - y, &mut image_vector, w3);
		setPoint(cxnum - x, cynum - y, &mut image_vector, w3);
		setPoint(cxnum + y, cynum + x, &mut image_vector, w3);
		setPoint(cxnum - y, cynum + x, &mut image_vector, w3);
		setPoint(cxnum + y, cynum - x, &mut image_vector, w3);
		setPoint(cxnum - y as i32, cynum - x as i32, &mut image_vector, w3);
	}

	//	draw_grid(&w, &mut image_vector);
	//	draw_circle(cxnum, cynum, radiusnum, w, &mut image_vector);

	//  write image
	imagefmt::Image{w: w, h: h, fmt:ColFmt::RGB, buf:image_vector}.write("out.png", ColType::Color).unwrap();

}

//fn draw_grid(w: i32, image_vector: &mut Vec<u8>){
//	let mut xc = 0;
//	let mut yc = 0;
//	let w3 = w*3;
//
//	let mut line_counter = 0;
//	for i in 0..image_vector.len() {
//		if i % (w3) == 0 {
//			for j in 0..201{
//				let new_i = (j * 6 * 3) + i;
//				image_vector[new_i] = 0;
//				image_vector[new_i+1] = 0;
//				image_vector[new_i+2] = 0;
//			}
//
//			if line_counter == 0{
//				for j in 0..w3{
//					image_vector[j + i] = 0;
//					xc += 1;
//
////					if yc == cynum && xc == cxnum {
////						for k in 1..5 {
////							let shift_factor = k * (w3) + i + j;
////							for l in 0..16 {
////								image_vector[ shift_factor + l] = 0;
////							}
////						}
////					}
//				}
//				xc = 0
//			}
//			line_counter += 1;
//
//			if line_counter == 6 {
//				line_counter = 0;
//			}
//			yc += 1;
//		}
//	}
//}

//fn draw_circle(x0: i32, cynum: i32, radius: i32, w: i32, image_vector: Vec<u8>) {
//
//	let mut f: i32 = (1 - radius);
//	let mut ddF_x: i32 = 0;
//	let mut ddF_y: i32 = (-2 * radius);
//	let mut x: i32 = 0;
//	let mut y: i32 = radius;
//
//	// Draw right, left, top and bottom point
////	println!("{} : {}", x0, (y0 + radius));
////	println!("{} : {}", x0, (y0 - radius));
////	println!("{} : {}", (x0 +radius), y0);
////	println!("{} : {}", (x0 - radius), y0);
//
//	setPoint(x0, y0 + radius, image_vector);
//	setPoint(x0, y0 - radius, image_vector);
//	setPoint(x0 + radius, y0, image_vector);
//	setPoint(x0 - radius, y0, image_vector);
//
//	while x < y {
//		if f >= 0 {
//			y -= 1;
//			ddF_y += 2;
//			f += ddF_y;
//		}
//		x += 1;
//		ddF_x += 2;
//		f += ddF_x + 1;
//
////		println!("{} : {}", (x0 + x), (y0 + y));
////		println!("{} : {}", (x0 - x), (y0 + y));
////		println!("{} : {}", (x0 + x), (y0 - y));
////		println!("{} : {}", (x0 - x), (y0 - y));
////		println!("{} : {}", (x0 + y), (y0 + x));
////		println!("{} : {}", (x0 - y), (y0 + x));
////		println!("{} : {}", (x0 + y), (y0 - x));
////		println!("{} : {}", (x0 - y), (y0 - x));
//
//		setPoint(x0 + x, y0 + y, image_vector);
//		setPoint(x0 - x, y0 + y, image_vector);
//		setPoint(x0 + x, y0 - y, image_vector);
//		setPoint(x0 - x, y0 - y, image_vector);
//		setPoint(x0 + y, y0 + x, image_vector);
//		setPoint(x0 - y, y0 + x, image_vector);
//		setPoint(x0 + y, y0 - x, image_vector);
//		setPoint(x0 - y, y0 - x, image_vector);
//	}
//}
//
fn setPoint(x: i32, y: i32, image_vector: &mut Vec<u8>, w3: i32){
	if x<=200 && y<=150 && x > 0 && y > 0  {
		let x_2d = (x * 3 + (x - 1) * 15 - 1);
		let y_2d = (y + (y - 1)* 5);
		let offset = (y_2d * w3);

		for i in 0..5 {
			for j in 1..16 {
				image_vector[(offset + x_2d + j + (i * w3)) as usize] = 0;
			}
		}
	}
}
