use crate::{
    format_node::{FormatNode, WrapArguments},
    transform::transform_rules::{get, post_visit, pre_visit},
};

use parser::tree::Tree;

#[derive(Debug, Default)]
pub(crate) struct FormatArguments {
    pub space: bool,
    pub newline: bool,
    pub double_newline: bool, // ignores newline - TODO better way to model this?
    pub indent: bool,         // TODO do we need a separate one for wrap and indent?
    pub wrap: bool,
    pub prevent_wrap_cascade: bool,
    pub child_wrap_prevents_wrap: bool,
}

pub fn transform<'source>(node: &Tree<'source>) -> FormatNode {
    let parent_name = node.name();

    #[derive(Debug)]
    struct FormatContainer {
        children: Vec<FormatNode>,
        wrapping: Option<WrapArguments>,
    }

    // TODO consider writing using TreeCursor
    if node.children().len() > 0 {
        // TODO prevent double wrap with function parameters
        // TODO for WrapIfChild should we have a wrap boundary?
        let mut stack: Vec<FormatContainer> = vec![FormatContainer {
            children: Vec::new(),
            wrapping: None,
        }];

        let mut stack_pushers_depth: Vec<usize> = vec![];

        let mut between = FormatArguments::default();

        // TODO create a local pop function

        for index in 0..node.children().len() {
            let child = &node.children()[index];
            let child_name = child.name();

            // preprocess
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|prev_index| node.children().get(prev_index))
            {
                let previous_name = previous.name();

                pre_visit(parent_name, &mut between, child_name, previous_name);
            }

            if between.wrap {
                let previous = stack.pop().unwrap();
                if let Some(wrapping) = previous.wrapping {
                    stack.last_mut().unwrap().children.push(FormatNode::Wrap(
                        FormatNode::Group(previous.children).into(),
                        wrapping,
                    ));
                    // stack.last_mut().unwrap().children.push(FormatNode::Group(previous.children));
                } else {
                    stack.push(FormatContainer {
                        children: previous.children,
                        wrapping: previous.wrapping,
                    }); // TODO makes sense?
                }
                stack.push(FormatContainer {
                    children: Vec::new(),
                    wrapping: Some(WrapArguments {
                        child_wrap_prevents_wrap: between.child_wrap_prevents_wrap,
                        wrap_with_indent: between.indent,
                        or_space: between.space,
                    }),
                });

                // TODO condense nested groups?
            }

            // stack push
            if let Some(previous) = index
                .checked_sub(1)
                .and_then(|prev_index| node.children().get(prev_index))
            {
                if get().stack_pushers.contains(previous.name())
                    && !get().stack_poppers.contains(child_name)
                {
                    stack_pushers_depth.push(stack.len());

                    stack.push(FormatContainer {
                        children: Vec::new(),
                        wrapping: None,
                    });
                }
            }

            // process
            let processed = transform(&child);

            match between {
                FormatArguments { wrap: true, .. } => {}
                FormatArguments {
                    double_newline: true,
                    ..
                } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }
                FormatArguments { indent: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline);
                }
                FormatArguments { newline: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Newline)
                }
                FormatArguments { space: true, .. } => {
                    stack.last_mut().unwrap().children.push(FormatNode::Space)
                }
                _ => {}
            }

            stack.last_mut().unwrap().children.push(match between {
                // TODO how do we capture the right nodes in this? Do we just keep capturing until we reach the next wrap node?
                // FormatArguments { wrap: true, ..} => FormatNode::Wrap(WrapArguments {
                //     wrap_with_indent: between.indent,
                //     or_space: between.space,
                // }),
                FormatArguments { wrap: true, .. } => processed,
                FormatArguments { indent: true, .. } => FormatNode::Indent(processed.into()),
                _ => processed,
            });

            between = FormatArguments::default();

            // stack pop
            if let Some(next) = node.children().get(index + 1) {
                if stack_pushers_depth.len() == 0 {
                    // Warn
                } else if get().stack_poppers.contains(next.name()) {
                    let expected_depth = stack_pushers_depth.pop().unwrap();

                    while stack.len() > expected_depth {
                        let last = stack.pop().unwrap();

                        stack.last_mut().unwrap().children.push(match last {
                            FormatContainer {
                                children,
                                wrapping: Some(wrapping),
                            } => FormatNode::Wrap(FormatNode::Group(children).into(), wrapping),
                            FormatContainer { children, .. } => FormatNode::Group(children),
                        });
                    }
                }
            }

            // postprocess
            if let Some(next) = node.children().get(index + 1) {
                let next_name = next.name();

                let has_multiple_newlines =
                    (next.range().start_point.row - child.range().end_point.row) > 1;

                post_visit(
                    parent_name,
                    &mut between,
                    child_name,
                    has_multiple_newlines,
                    next_name,
                );
            }
        }

        // TODO need to put the last item in a wrap as well?

        FormatNode::Group(
            stack
                .into_iter()
                .rev()
                .reduce(|child, mut parent| {
                    if let Some(wrap_arguments) = child.wrapping {
                        parent.children.push(FormatNode::Wrap(
                            FormatNode::Group(child.children).into(),
                            wrap_arguments,
                        ));
                    } else {
                        parent.children.push(FormatNode::Group(child.children));
                    }
                    return parent;
                })
                .unwrap()
                .children,
        )
    } else {
        let original_content = node.text();
        let content = if node.name() == "block_comment" {
            original_content
                .lines()
                .map(|line| line.trim_start())
                .enumerate()
                .map(|(index, line)| {
                    if index == 0 {
                        line.to_owned()
                    } else {
                        " ".to_owned() + line
                    }
                })
                .collect::<Vec<String>>()
                .join("\n")
        } else {
            original_content.into()
        };

        FormatNode::Content(content.into())
    }
}
