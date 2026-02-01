use crate::format_node::{FormatNode, WrapArguments};

pub struct PrettyPrintResult {
    pub result: String,
    // TODO should we split this struct between the methods since the meaning is slightly different for both?
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

#[derive(Debug, Default, Copy, Clone)] // TODO do we really need copy
pub struct WrapParameters {
    /** Whether to wrap because the content is too long */
    wrap_because_length: bool,
    /** Whether to wrap because some child is wrapped */
    wrap_because_child: bool,
    // /** Whether to wrap because the parent is wrapped */
    // wrap_because_parent: bool,
}

pub struct PrettyPrintParameters {
    pub(crate) indent_size: usize,
    pub(crate) max_line_length: usize,
}

// TODO we need to integrate indentations with this
const MAX_LINE_LENGTH: usize = 100;

pub fn prettyprint(formatted: &FormatNode, arguments: &PrettyPrintParameters, parent_wrap: WrapParameters) -> PrettyPrintResult {
    // println!("{:?}", formatted);

    // Simply print without checking line length - auto-wrapping is turned off
    let transformed = print(
        formatted,
        arguments,
        WrapParameters {
            wrap_because_length: false,
            wrap_because_child: false,
        },
        parent_wrap,
    );

    transformed
}

// TODO should we build the tree different to avoid needing the parent_wrap struct? I.e. two separate types?
// TODO should we have some identifiers for nodes to assist tracking?

// TODO analyze the tree based on computed widths before constructing strings and transform to processed tree for analysis?

fn print(
    formatted: &FormatNode,
    arguments: &PrettyPrintParameters,
    wrap: WrapParameters,
    parent_wrap: WrapParameters,
) -> PrettyPrintResult {
    // let wrap_children = wrap.wrap_because_length || wrap.wrap_because_child;

    match formatted {
        FormatNode::Content(content) => content.to_string().into(),
        FormatNode::Group(elements) => elements
            .iter()
            .map(|element| prettyprint(element, arguments, wrap))
            .reduce(|acc, item| PrettyPrintResult {
                result: acc.result + &item.result,
                is_wrapped: acc.is_wrapped || item.is_wrapped,
            })
            .unwrap(),
        FormatNode::Wrap(
            element,
            WrapArguments {
                wrap_with_indent,
                or_space,
                child_wrap_prevents_wrap,
                has_newline_in_source,
            },
        ) => {
            // Instead of auto-wrapping based on line length, honor existing newlines in source
            let should_wrap = *has_newline_in_source;
            let content = prettyprint(&element, arguments, wrap);
            let transformed_content = if should_wrap {
                "\n".to_owned()
                    + &if *wrap_with_indent {
                        indent(content.result, arguments.indent_size)
                    } else {
                        content.result
                    }
            } else {
                if *or_space {
                    " ".to_owned() + &content.result
                } else {
                    "".to_owned() + &content.result
                }
            };
            PrettyPrintResult {
                result: transformed_content,
                is_wrapped: content.is_wrapped && !*child_wrap_prevents_wrap,
            }
            // if parent_wrap {
            //     "\n".to_owned() + if *wrap_with_indent { INDENT } else { "" }
            // } else {
            //     if *or_space {
            //         " ".to_owned()
            //     } else {
            //         "".to_owned()
            //     }
            // }
            // .into()
        }
        FormatNode::Indent(element) => indent(prettyprint(element, arguments, wrap).result, arguments.indent_size).into(),
        FormatNode::Newline => "\n".to_owned().into(),
        FormatNode::Space => " ".to_owned().into(),
        // FormatNode::WrapBoundary(element) => PrettyPrintResult {
        //     result: prettyprint(element, wrap_children).result,
        //     is_wrapped: false,
        // },
    }
}

fn indent(content: String, indent_size: usize) -> String {
    content
        .lines()
        .map(|line| (" ".repeat(indent_size).to_owned()) + line)
        .collect::<Vec<String>>()
        .join("\n")
}
