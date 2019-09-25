use std::env;
use std::fs;

use glyphstool::{stretch, Font, FromPlist, Plist, ToPlist};

fn usage() {
    eprintln!("usage: glyphstool font.glyphs");
}

fn main() {
    let mut filename = None;
    for arg in env::args().skip(1) {
        if filename.is_none() {
            filename = Some(arg);
        }
    }
    if filename.is_none() {
        usage();
        return;
    }
    let filename = filename.unwrap();
    let contents = fs::read_to_string(filename).expect("error reading font");
    let plist = Plist::parse(&contents).expect("parse error");
    //println!("Plist: {:?}", plist);
    /*
    let font = Font::from_plist(plist);
    for glyph in font.glyphs() {
        println!("glyphname: {}", glyph.glyphname());
        for layer in glyph.layers() {
            println!("  layer: {}, width = {}", layer.layer_id(), layer.width());
        }
    }
    */
    let mut font: Font = FromPlist::from_plist(plist);
    //println!("{:?}", font);
    stretch(&mut font, 0.5, "051EFAE4-8BBE-4FBB-A016-4335C3E52F59");
    let plist = font.to_plist();
    println!("{}", plist.to_string());
}
