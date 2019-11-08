//! A bit of scripting to automate a bunch of Inconsolata vf work.
//!
//! Note that this is a submodule of main, rather than in the lib, as it is not
//! generally useful. But it's very likely that logic in here can be adapted into
//! a more general tool.

use std::collections::HashMap;

use kurbo::Affine;

use glyphstool::{Font, Glyph, Layer, Node, Path};

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

fn fix_path(path0: &Path, path1: &Path, t: f64, stretch: f64) -> Path {
    let a = affine_stretch(stretch);
    let nodes = path0
        .nodes
        .iter()
        .zip(path1.nodes.iter())
        .map(|(n0, n1)| Node {
            pt: a * n0.pt.lerp(n1.pt, t).round(),
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
            println!("  touching layer {}, t = {}", layer.layer_id, t);
            if let Some(ref p0) = paths0 {
                let paths = p0
                    .iter()
                    .zip(paths1.as_ref().unwrap().iter())
                    .map(|(p0, p1)| fix_path(p0, p1, t, stretch))
                    .collect();
                layer.paths = Some(paths);
            }
            layer.width = wdth as f64 * 5.0;

            // Possibly TODO: lerp the affine from the masters, rather than
            // doing the processing in-place. Not clear whether it makes much
            // difference.
            let a = affine_stretch(stretch);
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

pub fn inco_fix(font: &mut Font) {
    let mut layers = LayerMap::default();
    for master in &font.font_master {
        let wght = master.weight_value;
        let wdth = master.width_value.unwrap_or(100);
        println!("{}: wght {}, wdth {}", master.id, wght, wdth);
        layers.add(wght, wdth, &master.id);
    }
    let layer_400_narrow_id = layers.get_id(400, 50);
    for glyph in &mut font.glyphs {
        let narrow = glyph.get_layer(layer_400_narrow_id).unwrap();
        if narrow.width != 250. && !glyph.glyphname.starts_with("_corner") {
            fix_glyph(glyph, &layers);
        }
    }
}
