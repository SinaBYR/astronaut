// CSS box model
// All sizes are in px

use core::panic;

use super::style::{Display, StyledNode};

#[derive(Default)]
struct Dimensions {
    content: Rect,
    padding: EdgeSizes,
    margin: EdgeSizes,
    boder: EdgeSizes,
}

#[derive(Default)]
struct Rect {
    // x and y are px positions from document's origin
    x: f32,
    y: f32,
    width: f32,
    height: f32,
}

#[derive(Default)]
struct EdgeSizes {
    left: f32,
    right: f32,
    top: f32,
    bottom: f32,
}

struct LayoutBox {
    dimensions: Dimensions,
    box_type: BoxType,
    children: Vec<LayoutBox>,
}

impl LayoutBox {
    fn new(box_type: BoxType) -> LayoutBox {
        LayoutBox {
            dimensions: Default::default(),
            box_type,
            children: Vec::new(),
        }
    }

    fn generate_anonymous_box(&mut self) -> &mut LayoutBox {
        match self.box_type {
            BoxType::Inline | BoxType::Anonymous => self,
            BoxType::Block => {
                match self.children.last() {
                    Some(LayoutBox { box_type: BoxType::Anonymous, .. }) => {},
                    _ => self.children.push(LayoutBox::new(BoxType::Anonymous)),
                }
                self.children.last_mut().unwrap()
            }
        }
    }
}

enum BoxType {
    Block,
    Inline,
    Anonymous,
}

fn build_layout_tree(style_node: &StyledNode) -> LayoutBox {
    let mut root = LayoutBox::new(match style_node.display() {
        Display::Block  => BoxType::Block,
        Display::Inline => BoxType::Inline,
        Display::None   => panic!("root node has display: none;"),
    });

    for child in &style_node.children {
        match child.display() {
            Display::Block  => root.children.push(build_layout_tree(child)),
            // create an anonymous layout box
            // recursively transform styled_nodes to layout_boxes (DFS)
            Display::Inline => root.generate_anonymous_box().children.push(build_layout_tree(child)),
            Display::None   => {},
        }
    }

    root
}

