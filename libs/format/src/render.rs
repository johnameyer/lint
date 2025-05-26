use crate::format_node::{FormatNode, WrapArguments};

const INDENT: &str = "    ";

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

// TODO we need to integrate indentations with this
const MAX_LINE_LENGTH: usize = 100;

pub fn prettyprint(formatted: &FormatNode, parent_wrap: WrapParameters) -> PrettyPrintResult {
    // println!("{:?}", formatted);

    let transformed = print(
        formatted,
        WrapParameters {
            wrap_because_length: false,
            wrap_because_child: false,
        },
        parent_wrap,
    );

    let can_wrap = match formatted {
        FormatNode::Group(elements) => elements
            .iter()
            .any(|element| matches!(element, FormatNode::Wrap { .. })),
        _ => false,
    };

    if can_wrap
        && (transformed.is_wrapped
            || transformed
                .result
                .lines()
                .any(|line| line.len() > MAX_LINE_LENGTH))
    {
        let wrap_because_child = transformed.is_wrapped;
        let wrap_because_length = transformed
            .result
            .lines()
            .any(|line| line.len() > MAX_LINE_LENGTH);

        // TODO sometimes we fail to properly wrap a child, causing our parent to wrap when it shouldn't see unwrappable-child test example
        // This check against all children can potentially cause slowdowns as well

        let params = WrapParameters {
            wrap_because_length,
            wrap_because_child,
        };

        let new = print(formatted, params, parent_wrap);

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
            is_wrapped: transformed.is_wrapped,
        }
    }
}

// TODO should we build the tree different to avoid needing the parent_wrap struct? I.e. two separate types?
// TODO should we have some identifiers for nodes to assist tracking?

// TODO analyze the tree based on computed widths before constructing strings and transform to processed tree for analysis?

fn print(
    formatted: &FormatNode,
    wrap: WrapParameters,
    parent_wrap: WrapParameters,
) -> PrettyPrintResult {
    // let wrap_children = wrap.wrap_because_length || wrap.wrap_because_child;

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
        FormatNode::Wrap(
            element,
            WrapArguments {
                wrap_with_indent,
                or_space,
                child_wrap_prevents_wrap,
            },
        ) => {
            let should_wrap = parent_wrap.wrap_because_length || parent_wrap.wrap_because_child;
            let content = prettyprint(&element, wrap);
            let transformed_content = if should_wrap {
                "\n".to_owned()
                    + &if *wrap_with_indent {
                        indent(content.result)
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
        FormatNode::Indent(element) => indent(prettyprint(element, wrap).result).into(),
        FormatNode::Newline => "\n".to_owned().into(),
        FormatNode::Space => " ".to_owned().into(),
        // FormatNode::WrapBoundary(element) => PrettyPrintResult {
        //     result: prettyprint(element, wrap_children).result,
        //     is_wrapped: false,
        // },
    }
}

fn indent(content: String) -> String {
    content
        .lines()
        .map(|line| (INDENT.to_owned()) + line)
        .collect::<Vec<String>>()
        .join("\n")
}
