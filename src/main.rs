use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}

type AttrMap = HashMap<String, String>;

impl Node {
    fn new_text(data: String) -> Self {
        Self {
            children: Vec::new(),
            node_type: NodeType::Text(data),
        }
    }

    fn new_elem(name: String, attrs: AttrMap, children: Vec<Node>) -> Self {
        Self {
            children: children,
            node_type: NodeType::Element(ElementData{
                tag_name: name,
                attributes: attrs,
            }),
        }
    }
}


fn main() {
    println!("Hello, world!");
}
