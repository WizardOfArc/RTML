use rtml::{
    HtmlNode, new_empty_node,
    body, br, head, html, text,
    div
};
use std::collections::HashMap;

fn main() {
    println!("Hello, world!");
    let node = html(
        head("Azi page".to_string(), Vec::new()),
        body(
            vec!(
                new_empty_node("jack"),
                br(),
                text("Hi there"),
                text("I hope this works"),
                div(
                    [("class".to_string(), "groovy awesome".to_string())].iter().cloned().collect(),
                    vec!(text("I'm in a div"))
                ),
                new_empty_node("jill"),
            )
        )
    );
    println!("{}",node.render(0));
}
