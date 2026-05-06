use std::{
    fs::File,
    io::{self, BufWriter, Write},
};

use ppm_explore::{Vec2f32, Vector};

const HEIGHT: usize = 512;
const WIDTH: usize = 512;
const RANGE: usize = 255;

fn main() -> io::Result<()> {
    let mut output = BufWriter::new(
        File::options()
            .write(true)
            .truncate(true)
            .open("output.ppm")?,
    );
    output.write_ln("P6")?;
    output.write_ln(&format!("{WIDTH} {HEIGHT} {RANGE}"))?;
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let u = x as f32 / WIDTH as f32;
            let v = y as f32 / HEIGHT as f32;
            let color = empty_circle_frag(Vec2f32::new(u, v));
            write_color(color, &mut output)?;
        }
    }
    Ok(())
}

type Color = Vector<3, f32>;

fn write_color(color: Color, output: &mut BufWriter<File>) -> io::Result<()> {
    let [r, g, b] = color.inner();
    let r = (r * RANGE as f32).round() as u8;
    let g = (g * RANGE as f32).round() as u8;
    let b = (b * RANGE as f32).round() as u8;
    output.write_all(&[r, g, b])?;
    Ok(())
}

fn stripes_frag(uv: Vec2f32) -> Color {
    let n = 20.;
    Color::new(
        ((uv.x() * n).sin() + 1.) / 2.,
        (((uv.x() + uv.y()) * n).sin() + 1.) / 2.,
        ((uv.y() * n).cos() + 1.) / 2.,
    )
}

const FRAG_CENTER: Vec2f32 = Vec2f32::splat(0.5);

fn circle_frag(uv: Vec2f32) -> Color {
    const RADIUS: f64 = 0.25;
    let diff = FRAG_CENTER - uv;
    let r = (diff.length() < RADIUS) as u8 as f32;
    Color::new(r, 0.0, 0.0)
}

fn empty_circle_frag(uv: Vec2f32) -> Color {
    const OUTER_RADIUS: f64 = 0.25;
    const INNER_RADIUS: f64 = OUTER_RADIUS - 0.05;
    let len = (FRAG_CENTER - uv).length();
    let r = (len < OUTER_RADIUS && len > INNER_RADIUS) as u8 as f32;
    Color::new(r, 0.0, 0.0)
}

trait WriteLn {
    fn write_ln(&mut self, text: &str) -> io::Result<()>;
}

impl WriteLn for BufWriter<File> {
    fn write_ln(&mut self, text: &str) -> io::Result<()> {
        let text = String::new() + text + "\n";
        self.write_all(text.as_bytes())?;
        Ok(())
    }
}
