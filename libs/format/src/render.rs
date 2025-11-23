use crate::{calculate_width::{MaxWidthResult, get_longest_width}, format_node::{FormatNode, WrapArguments}};

pub struct PrettyPrintResult {
    pub result: String,
    // TODO should we split this struct between the methods since the meaning is slightly different for both?
    is_wrapped: bool,
    /// Whether there are elements that can wrap that are not wrapped
    can_wrap_further: bool,
}

impl From<String> for PrettyPrintResult {
    fn from(value: String) -> Self {
        PrettyPrintResult {
            result: value,
            is_wrapped: false,
            can_wrap_further: false,
        }
    }
}

#[derive(Debug, Default, Copy, Clone)] // TODO do we really need copy
pub struct WrapParameters {
    /// Whether to wrap because the content is too long - propogated downwards to the first wrap element
    wrap_because_length: bool,
    /// Whether to wrap because some child is wrapped
    wrap_because_child: bool,
    // Whether to wrap because the parent is wrapped - wrap elements refuse by setting the wrap_if_parent to false
    wrap_because_parent: bool,
}

pub struct PrettyPrintParameters {
    pub(crate) indent_size: usize,
    // TODO we need to integrate indentations with this
    pub(crate) max_line_length: usize,
}

pub fn prettyprint(formatted: &FormatNode, arguments: &PrettyPrintParameters, parent_wrap: WrapParameters) -> PrettyPrintResult {
    // println!("{:?}", formatted);
    // println!("{:X} {parent_wrap:?}", formatted.get_hash());

    let MaxWidthResult { max_width, can_wrap } = get_longest_width(formatted, false, arguments.indent_size);

    // TODO

    println!("{:?}", MaxWidthResult { max_width, can_wrap } );

    let transformed = print(
        formatted,
        arguments,
        WrapParameters {
            wrap_because_length: parent_wrap.wrap_because_length,
            wrap_because_child: false,
            wrap_because_parent: if let FormatNode::Wrap(_, WrapArguments {wrap_if_parent: false,  .. }) = formatted { false } else { parent_wrap.wrap_because_parent },
        },
        parent_wrap,
    );

    let should_wrap = max_width > arguments.max_line_length || transformed.is_wrapped;

    if can_wrap && should_wrap {
        let wrap_because_child = transformed.is_wrapped;
        let wrap_because_length = transformed
            .result
            .lines()
            .any(|line| line.len() > arguments.max_line_length);
        let wrap_because_parent = true;

        // TODO sometimes we fail to properly wrap a child, causing our parent to wrap when it shouldn't see unwrappable-child test example
        // This check against all children can potentially cause slowdowns as well
        // However, we need a strategy when the child still has characters on our line that we might want to wrap to avoid...
        // I.e. if-content-does-not-wrap

        // println!("Passing {wrap_because_parent}");

        let params = WrapParameters {
            wrap_because_length,
            wrap_because_child,
            wrap_because_parent,
        };

        // println!("Passing params {params:?}");

        let new = print(formatted, arguments, params, parent_wrap);

        if transformed.result.lines().any(|line| line.len() > arguments.max_line_length) {
            // panic!("Unexpectedly long line! {:?} {:?}", formatted, transformed);
        }

        PrettyPrintResult {
            result: new.result,
            is_wrapped: true,
            can_wrap_further: new.can_wrap_further,
        }
    } else {
        PrettyPrintResult {
            result: transformed.result,
            is_wrapped: transformed.is_wrapped,
            can_wrap_further: transformed.can_wrap_further,
        }
    }
}

// pub fn should_wrap(formatted: &FormatNode, arguments: &PrettyPrintParameters) -> bool {
//     // TODO should we be avoiding recursing the tree twice between this call and the group should_wrap recursive call
//     let MaxWidthResult { max_width, can_wrap } = get_longest_width(formatted, false, arguments.indent_size);
//     let parent_wrap = max_width > arguments.max_line_length;

//     // let has_wrapped_child = match formatted {
//     //     FormatNode::Content(_) | FormatNode::Indent(_) | FormatNode::Newline | FormatNode::Space => false,
//     //     FormatNode::Group(elements) => elements
//     //         .iter()
//     //         .any(|element| should_wrap(element, arguments)),
//     //     FormatNode::Wrap(
//     //         ..
//     //     ) => false
//     // };

//     parent_wrap && can_wrap
// }

// TODO should we build the tree different to avoid needing the parent_wrap struct? I.e. two separate types?
// TODO should we have some identifiers for nodes to assist tracking?

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
                can_wrap_further: acc.can_wrap_further || item.can_wrap_further,
            })
            .unwrap(),
        FormatNode::Wrap(
            element,
            WrapArguments {
                wrap_with_indent,
                or_space,
                child_wrap_prevents_wrap,
                wrap_if_parent,
            },
        ) => {
            let should_wrap = parent_wrap.wrap_because_length || parent_wrap.wrap_because_child || parent_wrap.wrap_because_parent && *wrap_if_parent;
            // Wrap elements only propogate downwards the wrap_because_parent flag
            let wrap_parameters = WrapParameters {
                wrap_because_length: false,
                wrap_because_child: false,
                wrap_because_parent: should_wrap,
            };
            let content = prettyprint(&element, arguments, wrap_parameters);
            // println!("Transforming wrap element {parent_wrap:?}");

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
                can_wrap_further: !should_wrap,
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
        FormatNode::Indent(element) => PrettyPrintResult {
            result: indent(prettyprint(element, arguments, wrap).result, arguments.indent_size).into(),
            is_wrapped: false,
            can_wrap_further: false
        },
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

#[cfg(test)]
mod tests {
    use super::*;
    use FormatNode::*;

    #[test]
    fn print_nested_wrapping() {
        // We expect the external myLongClassName.myLongFunctionName to be on one line and the inner parameters to only wrap onto a single newline
        // If it was myLongLongFunctionName, the entire string after the . would be too long to fit on a single string
        let expected = "myLongClassName.myLongFunctionName(\n  myReallyReallyStupidlyLongValueName, myReallyReallyStupidlyLongValueName\n)";
        // This is because the whole line would be too long
        // But since the childWrapPreventsWrap is enabled, that is preferred unless the content would still be too long

        // TODO
        let nodes = Group(vec![]);

        let formatted = prettyprint(&nodes, &PrettyPrintParameters { indent_size: 2, max_line_length: 80 }, WrapParameters::default()).result;

        assert_eq!(expected, formatted);
    }
}
