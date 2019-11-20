//! A bit of scripting to automate a bunch of Inconsolata vf work.
//!
//! Note that this is a submodule of main, rather than in the lib, as it is not
//! generally useful. But it's very likely that logic in here can be adapted into
//! a more general tool.

use std::collections::HashMap;

use kurbo::Affine;

use glyphstool::{Component, Font, Glyph, Layer, Node, NodeType, Path};

#[derive(Default)]
struct LayerMap {
    params_to_id: HashMap<(i64, i64), String>,
    id_to_params: HashMap<String, (i64, i64)>,
}

impl LayerMap {
    fn add(&mut self, wght: i64, wdth: i64, id: &str) {
        self.params_to_id.insert((wght, wdth), id.to_string());
        self.id_to_params.insert(id.to_string(), (wght, wdth));
    }

    fn get_id(&self, wght: i64, wdth: i64) -> &str {
        &self.params_to_id[&(wght, wdth)]
    }

    fn get_params(&self, id: &str) -> Option<(i64, i64)> {
        self.id_to_params.get(id).copied()
    }
}

fn affine_stretch(stretch: f64) -> Affine {
    Affine::new([stretch, 0., 0., 1., 0., 0.])
}

fn simple_lerp_path(path0: &Path, path1: &Path, t: f64) -> Path {
    let nodes = path0
        .nodes
        .iter()
        .zip(path1.nodes.iter())
        .map(|(n0, n1)| Node {
            pt: n0.pt.lerp(n1.pt, t),
            node_type: n0.node_type,
        })
        .collect();
    Path {
        closed: path0.closed,
        nodes,
    }
}

fn fix_path(path0: &Path, path1: &Path, t: f64, a: Affine) -> Path {
    let nodes = path0
        .nodes
        .iter()
        .zip(path1.nodes.iter())
        .map(|(n0, n1)| Node {
            pt: (a * n0.pt.lerp(n1.pt, t)).round(),
            node_type: n0.node_type,
        })
        .collect();
    Path {
        closed: path0.closed,
        nodes,
    }
}

fn fix_glyph(glyph: &mut Glyph, layers: &LayerMap) {
    let paths0 = glyph
        .get_layer(layers.get_id(400, 100))
        .unwrap()
        .paths
        .clone();
    // This is actually the 700 from the master, but is stored in 900.
    let paths1 = glyph
        .get_layer(layers.get_id(900, 100))
        .unwrap()
        .paths
        .clone();
    println!("processing glyph {}", glyph.glyphname);
    for layer in &mut glyph.layers {
        if let Some((wght, wdth)) = layers.get_params(&layer.layer_id) {
            let t = (wght as f64 - 400.0) / 300.0;
            let stretch = wdth as f64 / 100.0;
            let a = affine_stretch(stretch);
            println!("  touching layer {}, t = {}", layer.layer_id, t);
            if let Some(ref p0) = paths0 {
                let paths = p0
                    .iter()
                    .zip(paths1.as_ref().unwrap().iter())
                    .map(|(p0, p1)| fix_path(p0, p1, t, a))
                    .collect();
                layer.paths = Some(paths);
            }
            layer.width = wdth as f64 * 5.0;

            // Possibly TODO: lerp the affine from the masters, rather than
            // doing the processing in-place. Not clear whether it makes much
            // difference.
            let a_inv = affine_stretch(stretch.recip());

            if let Some(ref mut anchors) = layer.anchors {
                for anchor in anchors {
                    anchor.position = (a * anchor.position).round();
                }
            }

            if let Some(ref mut components) = layer.components {
                for component in components {
                    if let Some(ref mut transform) = component.transform {
                        // TODO: round the translation component
                        *transform = a * *transform * a_inv;
                    }
                }
            }
        }
    }
}

fn get_layer_map(font: &Font) -> LayerMap {
    let mut layers = LayerMap::default();
    for master in &font.font_master {
        let wght = master.weight_value;
        let wdth = master.width_value.unwrap_or(100);
        println!("{}: wght {}, wdth {}", master.id, wght, wdth);
        layers.add(wght, wdth, &master.id);
    }
    layers
}

pub fn inco_fix(font: &mut Font) {
    let layers = get_layer_map(font);
    let layer_400_narrow_id = layers.get_id(400, 50);
    for glyph in &mut font.glyphs {
        let narrow = glyph.get_layer(layer_400_narrow_id).unwrap();
        if narrow.width != 250. && !glyph.glyphname.starts_with("_corner") {
            fix_glyph(glyph, &layers);
        }
    }
}

/// Scaling of small alphanumerics follows

const NUM_PAIRS: &[(&str, &str)] = &[
    ("zero", "zerosuperior"),
    ("zero.ss02", "zerosuperior.ss02"),
    // Note: zero form isn't here because it's made by composition
    ("one", "onesuperior"),
    ("two", "twosuperior"),
    ("three", "threesuperior"),
    ("four", "foursuperior"),
    ("five", "fivesuperior"),
    ("six", "sixsuperior"),
    ("seven", "sevensuperior"),
    ("eight", "eightsuperior"),
    ("nine", "ninesuperior"),
];

const ORD_PAIRS: &[(&str, &str)] = &[
    ("a", "ordfeminine"),
    ("o", "ordmasculine"),
];

const FRACS: &[(&str, &str, &str)] = &[
    ("one", "two", "onehalf"),
    ("one", "four", "onequarter"),
    ("three", "four", "threequarters"),
];

fn lerp_layers(glyph: &Glyph, weight: f64, width: f64, a: Affine, layers: &LayerMap) -> Vec<Path> {
    let (wt0, wt1, wtt) = if weight < 400.0 {
        (200, 400, (weight - 200.0) / 200.0)
    } else {
        (400, 900, (weight - 400.0) / 500.0)
    };
    let (wd0, wd1, wdt) = if width < 100.0 {
        (50, 100, (width - 50.0) / 50.0)
    } else {
        (100, 200, (width - 100.0) / 100.0)
    };
    let paths00 = glyph
        .get_layer(layers.get_id(wt0, wd0))
        .unwrap()
        .paths
        .clone();
    let paths01 = glyph
        .get_layer(layers.get_id(wt0, wd1))
        .unwrap()
        .paths
        .clone();
    let paths10 = glyph
        .get_layer(layers.get_id(wt1, wd0))
        .unwrap()
        .paths
        .clone();
    let paths11 = glyph
        .get_layer(layers.get_id(wt1, wd1))
        .unwrap()
        .paths
        .clone();
    if let Some(ref p0) = paths00 {
        return p0
            .iter()
            .zip(paths10.as_ref().unwrap().iter())
            .zip(paths01.as_ref().unwrap().iter())
            .zip(paths11.as_ref().unwrap().iter())
            .map(|(((p00, p10), p01), p11)| {
                let p0 = simple_lerp_path(p00, p01, wdt);
                let p1 = simple_lerp_path(p10, p11, wdt);
                fix_path(&p0, &p1, wtt, a)
            })
            .collect();
    }
    // This shouldn't happen.
    Vec::new()
}

fn add_ord_dash(paths: &mut Vec<Path>, wght: i64, wdth: i64) {
    let mut path = Path::new(true);
    let thickness = match wght {
        200 => 24.,
        400 => 54.,
        900 => 107.,
        _ => panic!("unexpected weight"),
    };
    let thickness_fudge = match wdth {
        50 => 0.9,
        100 => 1.0,
        200 => 1.05,
        _ => panic!("unexpected width"),
    };
    let thickness = thickness * thickness_fudge;
    let mut xw = (wght as f64 - 400.0) * 0.025;
    if wdth == 200 {
        xw -= 25.0;
    }
    let x0 = wdth as f64 * 0.76 - xw;
    let x1 = wdth as f64 * 4.25 + xw;
    let yc = 195.0f64;
    let y0 = (yc - 0.7 * thickness).round();
    let y1 = (yc + 0.3 * thickness).round();
    path.add((x0, y0), NodeType::Line);
    path.add((x1, y0), NodeType::Line);
    path.add((x1, y1), NodeType::Line);
    path.add((x0, y1), NodeType::Line);
    paths.push(path);
}

fn add_fraction_ref(layer: &mut Layer) {
    let component = Component {
        name: "fraction".to_string(),
        transform: None,
        other_stuff: Default::default(),
    };
    layer.components = Some(vec![component]);
}

pub fn inco_scale(font: &mut Font, subcmd: i32) {
    let layers = get_layer_map(font);

    // This is very cut'n'pasty, reflecting the development process. Obviously this
    // would be cleaned up for a reusable tool.
    match subcmd {
        // small numerics
        0 => {
            for (src, dst) in NUM_PAIRS {
                println!("{} -> {}", src, dst);
                let src_glyph = font.get_glyph(src).expect("glyph not found");
                let mut glyph = src_glyph.clone();
                glyph.glyphname = dst.to_string();
                for layer in &mut glyph.layers {
                    if let Some((wght, wdth)) = layers.get_params(&layer.layer_id) {
                        let dst_wght = (wght as f64 * 1.3).min(1000.0);
                        let dst_wdth = wdth as f64 * 1.1;
                        let x = (wdth as f64 - dst_wdth * 0.62) * 5.0 * 0.5;
                        let a = Affine::new([0.62, 0.0, 0.0, 0.62, x, 246.0]);
                        let paths = lerp_layers(src_glyph, dst_wght, dst_wdth, a, &layers);
                        layer.paths = Some(paths);
                    }
                }
                let dst_glyph  = font.get_glyph_mut(dst).expect("dst glyph not found");
                glyph.other_stuff = dst_glyph.other_stuff.clone();
                *dst_glyph = glyph;
            }
        }
        // ordfeminine, ordmasculine
        1 => {
            for (src, dst) in ORD_PAIRS {
                println!("{} -> {}", src, dst);
                let src_glyph = font.get_glyph(src).expect("glyph not found");
                let mut glyph = src_glyph.clone();
                glyph.glyphname = dst.to_string();
                for layer in &mut glyph.layers {
                    if let Some((wght, wdth)) = layers.get_params(&layer.layer_id) {
                        let dst_wght = (wght as f64 * 1.2).min(1000.0);
                        let dst_wdth = wdth as f64 * 0.95;
                        let x = (wdth as f64 - dst_wdth * 0.77) * 5.0 * 0.5;
                        let a = Affine::new([0.77, 0.0, 0.0, 0.77, x, 267.0]);
                        let mut paths = lerp_layers(src_glyph, dst_wght, dst_wdth, a, &layers);
                        add_ord_dash(&mut paths, wght, wdth);
                        layer.paths = Some(paths);
                    }
                }
                let dst_glyph  = font.get_glyph_mut(dst).expect("dst glyph not found");
                glyph.other_stuff = dst_glyph.other_stuff.clone();
                *dst_glyph = glyph;
            }
        }
        // fractions
        2 => {
            for (num, denom, dst) in FRACS {
                println!("{} / {} -> {}", num, denom, dst);
                let num_glyph = font.get_glyph(num).expect("glyph not found");
                let denom_glyph = font.get_glyph(denom).expect("glyph not found");
                let mut glyph = num_glyph.clone();
                glyph.glyphname = dst.to_string();
                for layer in &mut glyph.layers {
                    if let Some((wght, wdth)) = layers.get_params(&layer.layer_id) {
                        let dst_wght = (wght as f64 * 1.5).min(1100.0);
                        let dst_wdth = wdth as f64 * 1.1;
                        let x = (wdth as f64 - dst_wdth * 0.49) * 5.0 * 0.5;
                        let dx = (wdth as f64) * 1.4;
                        let dx2 = match *denom {
                            "two" => dx * 0.93,
                            "four" => dx * 0.85,
                            _ => 0.0,
                        };
                        let a = Affine::new([0.49, 0.0, 0.0, 0.49, x - dx, 380.0]);
                        let mut paths = lerp_layers(num_glyph, dst_wght, dst_wdth, a, &layers);
                        let a = Affine::new([0.49, 0.0, 0.0, 0.49, x + dx2, -70.0]);
                        let denom_paths = lerp_layers(denom_glyph, dst_wght, dst_wdth, a, &layers);
                        paths.extend(denom_paths);
                        layer.paths = Some(paths);
                        add_fraction_ref(layer);
                    }
                }
                let dst_glyph  = font.get_glyph_mut(dst).expect("dst glyph not found");
                glyph.other_stuff = dst_glyph.other_stuff.clone();
                *dst_glyph = glyph;
            }
        }
        _ => {
            panic!("unknown subcmd");
        }
    }
}
