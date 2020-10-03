use std::collections::HashMap;

pub struct Node {
    pub children: Vec<Node>,
    pub node_type: NodeType,
}

pub enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

pub type AttrMap = HashMap<String, String>;

impl Node {
    pub fn new_text(data: String) -> Self {
        Self {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    pub fn new_elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Self {
            children: children,
            node_type: NodeType::Element(ElementData{
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}
