//! The general strategy is just to use a plist for storage. Also, lots of
//! unwrapping.
//!
//! There are lots of other ways this could go, including something serde-like
//! where it gets serialized to more Rust-native structures, proc macros, etc.

use std::collections::HashMap;

use kurbo::{Affine, Point};

use crate::from_plist::FromPlist;
use crate::plist::Plist;
use crate::to_plist::ToPlist;

#[derive(Debug, FromPlist, ToPlist)]
pub struct Font {
    pub glyphs: Vec<Glyph>,
    #[rest]
    pub other_stuff: HashMap<String, Plist>,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct Glyph {
    pub layers: Vec<Layer>,
    pub glyphname: String,
    #[rest]
    pub other_stuff: HashMap<String, Plist>,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct Layer {
    pub layer_id: String,
    pub width: f64,
    pub paths: Option<Vec<Path>>,
    pub components: Option<Vec<Component>>,
    pub anchors: Option<Vec<Anchor>>,
    pub guide_lines: Option<Vec<GuideLine>>,
    #[rest]
    pub other_stuff: HashMap<String, Plist>,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct Path {
    pub closed: bool,
    pub nodes: Vec<Node>,
}

#[derive(Debug)]
pub struct Node {
    pub pt: Point,
    pub node_type: NodeType,
}

#[derive(Debug)]
pub enum NodeType {
    Line,
    OffCurve,
    Curve,
    CurveSmooth,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct Component {
    pub name: String,
    pub transform: Option<Affine>,
    #[rest]
    pub other_stuff: HashMap<String, Plist>,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct Anchor {
    pub name: String,
    pub position: Point,
}

#[derive(Debug, FromPlist, ToPlist)]
pub struct GuideLine {
    pub angle: Option<f64>,
    pub position: Point,
}

impl FromPlist for Node {
    fn from_plist(plist: Plist) -> Self {
        let mut spl = plist.as_str().unwrap().split(' ');
        let x = spl.next().unwrap().parse().unwrap();
        let y = spl.next().unwrap().parse().unwrap();
        let pt = Point::new(x, y);
        let node_type = spl.next().unwrap().parse().unwrap();
        Node { pt, node_type }
    }
}

impl std::str::FromStr for NodeType {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LINE" => Ok(NodeType::Line),
            "OFFCURVE" => Ok(NodeType::OffCurve),
            "CURVE" => Ok(NodeType::Curve),
            "CURVE SMOOTH" => Ok(NodeType::CurveSmooth),
            _ => Err(format!("unknown node type {}", s)),
        }
    }
}

impl NodeType {
    fn glyphs_str(&self) -> &'static str {
        match self {
            NodeType::Line => "LINE",
            NodeType::OffCurve => "OFFCURVE",
            NodeType::Curve => "CURVE",
            NodeType::CurveSmooth => "CURVE SMOOTH",
        }
    }
}

impl ToPlist for Node {
    fn to_plist(self) -> Plist {
        format!(
            "{} {} {}",
            self.pt.x,
            self.pt.y,
            self.node_type.glyphs_str()
        )
        .into()
    }
}

impl FromPlist for Affine {
    fn from_plist(plist: Plist) -> Self {
        let raw = plist.as_str().unwrap();
        let raw = &raw[1..raw.len() - 1];
        let coords: Vec<f64> = raw.split(", ").map(|c| c.parse().unwrap()).collect();
        Affine::new([
            coords[0], coords[1], coords[2], coords[3], coords[4], coords[5],
        ])
    }
}

impl ToPlist for Affine {
    fn to_plist(self) -> Plist {
        let c = self.as_coeffs();
        format!(
            "{{{}, {}, {}, {}, {}, {}}}",
            c[0], c[1], c[2], c[3], c[4], c[5]
        )
        .into()
    }
}

impl FromPlist for Point {
    fn from_plist(plist: Plist) -> Self {
        let raw = plist.as_str().unwrap();
        let raw = &raw[1..raw.len() - 1];
        let coords: Vec<f64> = raw.split(", ").map(|c| c.parse().unwrap()).collect();
        Point::new(coords[0], coords[1])
    }
}

impl ToPlist for Point {
    fn to_plist(self) -> Plist {
        format!("{{{}, {}}}", self.x, self.y).into()
    }
}
