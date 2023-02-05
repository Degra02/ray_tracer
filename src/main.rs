use indicatif::ProgressBar;

mod vec3;

const WIDTH: i32 = 255;
const HEIGHT: i32 = 255;

fn main() {
   let pb = ProgressBar::new(HEIGHT as u64);
   println!("P3\n{} {}\n255", WIDTH, HEIGHT);
   for j in (0..HEIGHT).rev() {
       // eprintln!("\rScanlines remaining: {}", j);
       pb.inc(1);
       for i in 0..WIDTH {
            let r = ((i as f32 / (WIDTH - 1) as f32) * 255.999) as i32;
            let g = ((j as f32 / (HEIGHT - 1) as f32) * 255.999) as i32;
            let b = (0.25 * 255.999) as i32;

            println!("{} {} {}", r, g, b);
       }
   }
   pb.finish_with_message("Done");
}
