use clap::Parser;
use opencv;
use opencv::prelude::*;

mod circle;
mod population;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_name = "image_path")]
    image: String,
}

fn main() {
    let args = Args::parse();
    let img_read_mode = opencv::imgcodecs::ImreadModes::IMREAD_UNCHANGED as i32;
    let mut img = match opencv::imgcodecs::imread(args.image.as_str(), img_read_mode) {
        Ok(img) => img,
        Err(err) => panic!("Can't open the image: {:?}", err)
    };

    let individual = population::Individual::random(500, img.cols() as u16, img.rows() as u16);
    individual.draw(&mut img);

    match opencv::highgui::imshow("test", &img) {
        Err(err) => panic!("Can't display the image: {:?}", err),
        _ => ()
    };

    opencv::highgui::wait_key(0).unwrap();
}