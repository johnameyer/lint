use crate::{format_node::{FormatNode} };

#[derive(Debug)]
pub struct MaxWidthResult {
    /// The widest content child
    pub max_width: usize,
    /// Whether the content contains any potential wraps
    pub can_wrap: bool,
}

// TODO do we need to take in WrapParameters?
/// Gets the width of the longest line
/// 
/// Does not traverse into Indent blocks as wrapping does not cross that boundary
pub fn get_longest_width(
    node: &FormatNode,
    wrap: bool,
    indent_size: usize,
) -> MaxWidthResult {
    let WidthsResult { widths, can_wrap } = get_line_widths(node, wrap, indent_size);
    return MaxWidthResult { max_width: *widths.iter().max().unwrap(), can_wrap }
}

#[derive(Debug)]
struct WidthsResult {
    /// The widest content child
    widths: Vec<usize>,
    /// Whether the content contains any potential wraps
    can_wrap: bool,
}

fn get_line_widths(
    node: &FormatNode,
    wrap: bool,
    indent_size: usize,
) -> WidthsResult {
    match node {
        FormatNode::Content(content) => WidthsResult {
            widths: if content.len() == 0 { vec![0] } else { content.lines().map(|line| line.len()).collect() },
            can_wrap: false,
        },
        FormatNode::Group(format_nodes) => format_nodes.iter().map(|item| get_line_widths(item, wrap, indent_size)).enumerate().fold(WidthsResult { widths: vec![0], can_wrap: false }, |acc, (index, item)| {
            // 0 newlines -> 1 item in array, 1 newline -> 2 items in array
            // Add the last item of acc to the first item of acc
            let middle = acc.widths.last().unwrap() + item.widths.first().unwrap();
            let widths = acc.widths[0..acc.widths.len() - 1].into_iter().chain(vec![middle].iter()).chain(item.widths.iter().take(item.widths.len() - 1)).map(|item| *item).collect();
            WidthsResult { widths: widths, can_wrap: acc.can_wrap || item.can_wrap }
        }),
        // Indent is a boundary that wrapping does not happen across
        FormatNode::Indent(_) => WidthsResult { widths: vec![0, 0], can_wrap: false },
        FormatNode::Wrap(format_node, wrap_arguments) => {
            if wrap {
                // If we wrap and have wrap_with_indent, this node transforms into `\n${" " * indent_size}${format_node}`
                // So left is always 0
                let indentation = if wrap_arguments.wrap_with_indent { indent_size } else { 0 };
                let WidthsResult { mut widths, can_wrap: _} = get_line_widths(format_node, wrap, indent_size);
                // We add the indentation to the first line and push a new line
                widths[0] += indentation;
                widths.insert(0, 0);
                WidthsResult { widths, can_wrap: false }
            } else {
                let spacing = if wrap_arguments.or_space { 1 } else { 0 };
                let WidthsResult { mut widths, can_wrap: _} = get_line_widths(format_node, wrap, indent_size);
                widths[0] += spacing;
                // We add the space to the first line
                WidthsResult { widths, can_wrap: true }
            }
        },
        FormatNode::Space => WidthsResult { widths: vec![1], can_wrap: false },
        FormatNode::Newline => WidthsResult { widths: vec![0, 0], can_wrap: false },
    }
}


        // FormatNode::Group(format_nodes) => format_nodes.iter().map(|item| get_longest_width(item, wrap, indent_size)).fold(WrapResult { max_width: 0, can_wrap: false}, |acc, item| 
        //     WrapResult { max_width: acc.max_width + item.max_width, can_wrap: acc.can_wrap || item.can_wrap }
        // ),


        // // Indent is a boundary that wrapping does not happen across
        // FormatNode::Indent(format_node) => WrapResult { max_width: 0, can_wrap: false },

        // FormatNode::Space => WrapResult { max_width: 1, can_wrap: false },
        // FormatNode::Newline => WrapResult { max_width: 0, can_wrap: false },

// // TODO better names


// /// Returns None if there are any newlines
// fn get_intrinsic_width_or(
//     node: &FormatNode,
// ) -> Option<usize> {
//     match node {
//         FormatNode::Content(content) => content.len(),
//         FormatNode::Group(format_nodes) => format_nodes.iter().any(f),
//         FormatNode::Indent(format_node) => todo!(),
//         FormatNode::Wrap(format_node, wrap_arguments) => todo!(),
//         FormatNode::Space => todo!(),
//         FormatNode::Newline => todo!(),
//     }
//     .unwrap_or(default)
// }