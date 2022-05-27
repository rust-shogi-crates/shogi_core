use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("square_consts.rs");
    let mut dest_file = fs::File::create(dest_path)?;
    writeln!(
        dest_file,
        "// Constants in Square
#[doc(hidden)]
impl Square {{"
    )?;
    for index in 0..81 {
        let file = index / 9 + 1;
        let rank = index % 9;
        writeln!(
            dest_file,
            "    pub const SQ_{}{}: Square = unsafe {{ Square::from_u8_unchecked({}) }};",
            file,
            (rank + b'A') as char,
            index + 1,
        )?;
    }
    dest_file.write_all(
        b"}

/// Constants of type [`Square`].
pub mod consts {
    use super::Square;
",
    )?;
    for index in 0..81 {
        let file = index / 9 + 1;
        let rank = index % 9;
        write!(
            dest_file,
            "    /// Also available as `Square::SQ_{}{}`.
    pub const SQ_{}{}: Square = unsafe {{ Square::from_u8_unchecked({}) }};
",
            file,
            (rank + b'A') as char,
            file,
            (rank + b'A') as char,
            index + 1,
        )?;
    }
    dest_file.write_all(b"}\n")?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
