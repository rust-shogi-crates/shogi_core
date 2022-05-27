use std::env;
use std::fs;
use std::io::Write;
use std::path::Path;

fn generate_piece_consts(dest_file: &mut fs::File) -> std::io::Result<()> {
    dest_file.write_all(
        b"use crate::{Color, Piece, PieceKind};
// Constants in Piece
#[doc(hidden)]
impl Piece {
",
    )?;
    let color_table = [("B", "Black"), ("W", "White")];
    let piece_table = [
        ("P", "Pawn"),
        ("L", "Lance"),
        ("N", "Knight"),
        ("S", "Silver"),
        ("G", "Gold"),
        ("B", "Bishop"),
        ("R", "Rook"),
        ("K", "King"),
        ("PP", "ProPawn"),
        ("PL", "ProLance"),
        ("PN", "ProKnight"),
        ("PS", "ProSilver"),
        ("PB", "ProBishop"),
        ("PR", "ProRook"),
    ];
    for (color_short_name, color_ident) in color_table {
        for (piece_short_name, piece_ident) in piece_table {
            writeln!(
                dest_file,
                "    pub const {}_{}: Piece = Piece::new(PieceKind::{}, Color::{});",
                color_short_name, piece_short_name, piece_ident, color_ident,
            )?;
        }
    }
    dest_file.write_all(
        b"}

/// Constants of type [`Piece`].
pub mod piece {
    use super::*;
",
    )?;
    for (color_short_name, color_ident) in color_table {
        for (piece_short_name, piece_ident) in piece_table {
            writeln!(
                dest_file,
                "    /// Also available as `Piece::{}_{}`.
    pub const {}_{}: Piece = Piece::new(PieceKind::{}, Color::{});",
                color_short_name,
                piece_short_name,
                color_short_name,
                piece_short_name,
                piece_ident,
                color_ident,
            )?;
        }
    }
    dest_file.write_all(b"}\n")?;
    Ok(())
}

fn generate_square_consts(dest_file: &mut fs::File) -> std::io::Result<()> {
    writeln!(
        dest_file,
        "use crate::Square;
// Constants in Square
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
pub mod square {
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
    Ok(())
}

fn main() -> std::io::Result<()> {
    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("piece_consts.rs");
    let mut dest_file = fs::File::create(dest_path)?;
    generate_piece_consts(&mut dest_file)?;
    let dest_path = Path::new(&out_dir).join("square_consts.rs");
    let mut dest_file = fs::File::create(dest_path)?;
    generate_square_consts(&mut dest_file)?;
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
