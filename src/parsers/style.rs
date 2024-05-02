use std::collections::HashMap;

use super::{css::{Rule, Selector, StyleSheet, Value}, html::{ElementData, Node, NodeType}};

type PropertyMap = HashMap<String, Value>;

struct StyledNode<'a> {
    node: &'a Node,
    css_properties: PropertyMap,
    children: Vec<StyledNode<'a>>,
}

fn matches_selector(elem: &ElementData, selector: &Selector) -> bool {
    // if we had more selector types, we would've used matches syntax

    // 1. check tag name
    if selector.tag_name.iter().any(|tag_name| *tag_name != elem.tag_name) {
        return false;
    }

    // 2. check id
    if selector.id.iter().any(|id| elem.id() != Some(id)) {
        return false;
    }

    // 3. check class names
    let class_list = elem.classes();
    if selector.class.iter().any(|class| !class_list.contains(&**class)) {
        return false;
    }

    // no non-matching selector component was found
    true
}

fn matching_rules(elem: &ElementData, stylesheet: &mut StyleSheet) -> Vec<Rule> {
    stylesheet.rules
        .drain(..)
        .filter(|rule| matches_selector(elem, &rule.selector))
        .collect()
}

fn specified_values(elem: &ElementData, stylesheet: &mut StyleSheet) -> PropertyMap {
    // TODO: 1. inherit inherit-able parent css properties (tip in article)
    // TODO: 2. apply inline css declarations for each element (tip in article)
    let mut values = HashMap::new();
    let rules = matching_rules(elem, stylesheet);

    for rule in rules {
        for declaration in rule.declarations {
            values.insert(declaration.name, declaration.value);
        }
    }

    values
}

pub fn style_tree<'a>(root: &'a Node, stylesheet: &mut StyleSheet) -> StyledNode<'a> {
    StyledNode {
        node: root,
        css_properties: match root.node_type {
            NodeType::Element(ref elem) => specified_values(elem, stylesheet),
            _ => HashMap::new(),
        },
        children: root.children.iter().map(|child| style_tree(child, stylesheet)).collect(),
    }
}

