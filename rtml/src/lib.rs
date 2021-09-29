use std::collections::HashMap;
use std::iter::Map;

pub struct HtmlNode {
    name: String,
    attributes: HashMap<String, String>,
    self_closing: bool,
    children: Vec<HtmlNode>,
    text_content: String,
    text_node: bool,
}

pub fn text(raw_text: &str) -> HtmlNode {
    HtmlNode{
        name: "text".to_string(),
        attributes: HashMap::new(),
        self_closing: true,
        children: Vec::new(),
        text_content: raw_text.to_string(),
        text_node: true
    }
}

fn padding(indent: u8) -> String {
    let mut pad = String::from("");
    for _ in 1..=indent {
        pad.push_str("    ");
    }
    pad
}

impl HtmlNode {
    pub fn render(&self, indent: u8) -> String {
        let open = format!("<{}", self.name);
        let mut att_list = String::from(" ");
        for (key, value) in &(self.attributes){
            att_list.push_str(key);
            att_list.push('=');
            att_list.push('"');
            att_list.push_str(value);
            att_list.push('"');
            att_list.push(' ');
        }
        let mut close = "X".to_string();
        if self.text_node {
            format!("{}{}", padding(indent), self.text_content)
        } else if self.self_closing {
            close = "/>".to_string();
            format!("{}{}{}{}", padding(indent), open, att_list, close)
        } else {
            close = format!("</{}>", self.name);
            let rendered_children: String =
                (&self.children).into_iter()
                    .map(|node| {node.render(indent + 1)})
                    .collect::<Vec<String> >()
                    .join("\n");
            format!(
                "{}{}{}>\n{}\n{}{}",
                padding(indent),
                open,
                att_list,
                rendered_children,
                padding(indent),
                close
            )
        }
    }
}

pub fn new_empty_node(p_name: &str) -> HtmlNode {
    HtmlNode{
        name: p_name.to_string(),
        attributes: HashMap::new(),
        self_closing: false,
        children: Vec::new(),
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn new_node(p_name: &str, attributes: HashMap::<String, String>, children: Vec::<HtmlNode>) -> HtmlNode {
    HtmlNode{
        name: p_name.to_string(),
        attributes,
        self_closing: false,
        children,
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn html(head: HtmlNode, body: HtmlNode) -> HtmlNode {
    HtmlNode{
        name: "html".to_string(),
        attributes: HashMap::new(),
        self_closing: false,
        children: vec!(head, body),
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn head(title: String, children: Vec::<HtmlNode>) -> HtmlNode {
    let mut new_children = Vec::new();
    let title_node = HtmlNode{
        name: "title".to_string(),
        attributes: HashMap::new(),
        self_closing: false,
        children: Vec::new(),
        text_content: title,
        text_node: true,
    };
    new_children.push(title_node);
    for c in children {
        new_children.push(c);
    }
    HtmlNode{
        name: "head".to_string(),
        attributes: HashMap::new(),
        self_closing: false,
        children: new_children,
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn body(children: Vec::<HtmlNode>) -> HtmlNode {
    HtmlNode {
        name: "body".to_string(),
        attributes: HashMap::new(),
        self_closing: false,
        children,
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn div(attributes: HashMap::<String, String>, children: Vec::<HtmlNode>) -> HtmlNode {
    HtmlNode {
        name: "div".to_string(),
        attributes,
        self_closing: false,
        children,
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn new_self_closing_node(p_name: &str, attributes: HashMap::<String, String>) -> HtmlNode {
    HtmlNode {
        name: p_name.to_string(),
        attributes,
        self_closing: true,
        children: Vec::new(),
        text_content: String::from(""),
        text_node: false,
    }
}

pub fn img(src: String, attributes: HashMap::<String, String>) -> HtmlNode {
    let mut new_map = HashMap::new();
    new_map.insert("src".to_string(), src);
    for (k, v) in &attributes {
        new_map.insert(k.to_string(), v.to_string());
    }
    new_self_closing_node (
        "img",
        new_map
    )
}

pub fn br() -> HtmlNode {
    new_self_closing_node (
        "br", HashMap::new()
    )
}