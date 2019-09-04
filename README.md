# ublocks-ascii-art
![alt](https://i.imgur.com/MUKmCeR.png)  
ASCII-art generator using unicode block characters.

### Usage (*as binary*)
[*you need to have a rust toolchain set up*](https://rustup.rs/)
```
$ git clone https://github.com/NyxCode/ublocks-ascii-art
$ cd ./ublocks-ascii-art/
$ cargo run text
```

### Usage (*as library*)
```rust
fn main() -> Result<(), SomeErr> {
    let rendered = render_text::<SimpleRaster>(&"hey!".to_owned(), Font::BadScript, (100.0, 50.0))?;
    println!("{}", rendered);

    Ok(())
}
```

### Fonts
By default, BadScript is used as font - however, also `Roboto` and `Roboto Mono` can also be used without additionally setup:
```$xslt
[dependencies.ublock-ascii-art]
version = "0.1.0"
default-features = false
features = ["font-badscript", "font-rotobo", "font-roboto-mono"]
```
Of course, custom fonts can be used too. Just load them (see [rusttype example](https://gitlab.redox-os.org/redox-os/rusttype/blob/master/examples/simple.rs)) and wrap it in `Font::Custom(..)`.

### Rotations
Since unicode block elements can't just be rotated by rearranging the characters, rotation is built into the library.
`render_text` accepts a generic argument which specifies the underlying `Raster`. If you want to rotate the image by 90°, for example, use `RotatedRaster90`:
```rust
let rendered = render_text::<RotatedRaster90<SimpleRaster>>(&"Hey!".to_owned(), Font::BadScript, (100.0, 50.0))?;
```
![alt](https://i.imgur.com/o6hHcGF.png)