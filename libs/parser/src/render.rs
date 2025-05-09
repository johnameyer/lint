use crate::format_node::{FormatNode, WrapArguments};

const INDENT: &str = "    ";

pub struct PrettyPrintResult {
    pub result: String,
    is_wrapped: bool,
}

impl From<String> for PrettyPrintResult {
    fn from(value: String) -> Self {
        PrettyPrintResult {
            result: value,
            is_wrapped: false,
        }
    }
}

const max_line_length: usize = 100;

pub fn prettyprint(formatted: &FormatNode, parent_wrap: bool) -> PrettyPrintResult {
    // println!("{:?}", formatted);

    let transformed = print(formatted, false, parent_wrap);

    let can_wrap = match formatted {
        FormatNode::Group(elements) => elements
            .iter()
            .any(|element| matches!(element, FormatNode::Wrap { .. })),
        _ => false,
    };

    // can only wrap if any children are wrapped

    if can_wrap && (transformed.is_wrapped || transformed.result.lines().any(|line| line.len() > max_line_length)) {
        let new = print(formatted, true, parent_wrap);

        if transformed.result.lines().any(|line| line.len() > 100) {
            // panic!("Unexpectedly long line! {:?} {:?}", formatted, transformed);
        }

        PrettyPrintResult {
            result: new.result,
            is_wrapped: true,
        }
    } else {
        PrettyPrintResult {
            result: transformed.result,
            is_wrapped: false,
        }
    }
}

fn print(formatted: &FormatNode, wrap: bool, parent_wrap: bool) -> PrettyPrintResult {
    match formatted {
        FormatNode::Content(content) => content.to_string().into(),
        FormatNode::Group(elements) => elements
            .iter()
            .map(|element| prettyprint(element, wrap))
            .reduce(|acc, item| PrettyPrintResult {
                result: acc.result + &item.result,
                is_wrapped: acc.is_wrapped || item.is_wrapped,
            })
            .unwrap(),
        FormatNode::Wrap(WrapArguments {
            wrap_with_indent,
            or_space,
        }) => {
            // if parent_wrap {
            //     "\n".to_owned() + &if *wrap_with_indent { indent(prettyprint(&element, wrap).result) } else { "".to_owned() }
            // } else {
            //     if *or_space {
            //         " ".to_owned()
            //     } else {
            //         "".to_owned()
            //     }
            // }
            // .into(),
            if parent_wrap {
                "\n".to_owned() + if *wrap_with_indent { INDENT } else { "" }
            } else {
                if *or_space {
                    " ".to_owned()
                } else {
                    "".to_owned()
                }
            }
            .into()
        }
        FormatNode::Indent(element) => indent(prettyprint(element, wrap).result).into(),
        FormatNode::Newline => "\n".to_owned().into(),
        FormatNode::Space => " ".to_owned().into(),
        FormatNode::WrapBoundary(element) => PrettyPrintResult {
            result: prettyprint(element, wrap).result,
            is_wrapped: false,
        },
        _ => "".to_string().into(),
    }
}

fn indent(content: String) -> String {
    content
        .lines()
        .map(|line| (INDENT.to_owned()) + line)
        .collect::<Vec<String>>()
        .join("\n")
}
