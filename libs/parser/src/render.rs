use crate::format_node::FormatNode;

const INDENT: &str = "    ";

pub fn prettyprint(formatted: &FormatNode, parent_wrap: bool) -> String {
    // println!("{:?}", formatted);

    let transformed = print(formatted, false, parent_wrap);

    if transformed.lines().any(|line| line.len() > 100) {
        let new = print(formatted, true, parent_wrap);

        if transformed.lines().any(|line| line.len() > 100) {
            // panic!("Unexpectedly long line! {:?} {:?}", formatted, transformed);
        }

        new
    } else {
        transformed
    }
}

fn print(formatted: &FormatNode, wrap: bool, parent_wrap: bool) -> String {
    match formatted {
        FormatNode::Content(content) => content.to_string(),
        FormatNode::Group(elements) => elements
            .iter()
            .map(|element| prettyprint(element, wrap))
            .collect(),
        // FormatNode::Indent(elements) => {
        //     if elements.len() > 3 {
        //         let transformed = elements
        //             .iter()
        //             .map(|element| prettyprint(element, wrap))
        //             .collect::<String>();

        //         let as_lines = transformed.lines().collect::<Vec<&str>>();

        //         let len = as_lines.len();

        //         as_lines
        //             .into_iter()
        //             .enumerate()
        //             .map(|(i, line)| {
        //                 if i == 0 || i == len - 1 {
        //                     line.to_string()
        //                 } else {
        //                     (INDENT.to_owned() + line).to_owned()
        //                 }
        //             })
        //             .collect::<Vec<String>>()
        //             .join("\n")
        //             .to_owned()
        //     } else {
        //         "{ }".to_owned()
        //     }
        // }
        FormatNode::Wrap {
            wrap_with_indent,
            or_space,
        } => {
            if parent_wrap {
                "\n".to_owned() + if *wrap_with_indent { INDENT } else { "" }
            } else {
                if *or_space {
                    " ".to_owned()
                } else {
                    "".to_owned()
                }
            }
        }
        // TODO when we indent, we need to indent all of the next element
        FormatNode::Indent(element) => prettyprint(element, wrap)
            .lines()
            .map(|line| (INDENT.to_owned()) + line)
            .collect::<Vec<String>>()
            .join("\n"),
        FormatNode::Newline => "\n".to_owned(),
        FormatNode::Space => " ".to_owned(),
        _ => "".to_string(),
    }
}
