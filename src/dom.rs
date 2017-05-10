#![allow(dead_code)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Node {
    children: Vec<Node>,
    node_type: NodeType
}



#[derive(Debug)]
pub enum NodeType {
    Text(String),
    Element(ElementData)
}

#[derive(Debug)]
pub struct ElementData {
    tag_name: String,
    attributes: AttrMap
}


type AttrMap = HashMap<String, String>;


pub fn text(data: String) -> Node {
    Node { children: Vec::new(), node_type: NodeType::Text(data) }
}

pub fn elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Node {
    Node {
        children: children,
        node_type: NodeType::Element(ElementData{
            tag_name: name,
            attributes: attrs
        })
    }
}
