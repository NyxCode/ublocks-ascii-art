use ublocks_ascii_art::*;

fn main() -> Result<(), SomeErr> {
    let text = std::env::args().skip(1).next().unwrap_or_else(|| "Hey!".to_owned());
    let rendered = render_text::<SimpleRaster>(&text,
                                               Font::BadScript,
                                               (200.0, 100.0))?;
    println!("{}", rendered);

    Ok(())
}