use ublocks_ascii_art::*;

fn main() -> Result<(), SomeErr> {
    let text = std::env::args().skip(1).next().unwrap_or_else(|| "Hey!".to_owned());
    let rendered = render_text::<RotatedRaster90<SimpleRaster>>(&text,
                                               Font::BadScript,
                                               (90.0, 200.0))?;
    println!("{}", rendered);

    Ok(())
}