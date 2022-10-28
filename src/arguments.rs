use clap::Parser;

#[derive(Parser, Debug)]
pub struct Args {
    /// Name of the output image file
    #[arg(short, long, default_value_t = format!("out.ppm"))]
    pub out_file: String,

    /// Width of the image
    #[arg(short, long, default_value_t = 400)]
    pub image_width: u32,

    /// Numerator of the aspect ratio
    #[arg(short, long, default_value_t = 16.0)]
    pub numerator_ar: f64,

    /// Denominator of the aspect ratio
    #[arg(short, long, default_value_t = 9.0)]
    pub denominator_ar: f64
}

pub fn parse_command_line_args() -> Args {
    Args::parse()
}