use lazy_static::lazy_static;
use std::fmt::Display;

pub struct Symbols {
    /// horizontal bar
    h_bar: char,
    /// vertical bar
    v_bar: char,
    /// x crossing bar
    x_bar: char,
    /// left crossing bar
    l_cross: char,
    /// right crossing bar
    r_cross: char,
    /// left bottom
    l_bot: char,
    /// right bottom
    r_bot: char,
    /// left top
    l_top: char,
    /// right top
    r_top: char,
    /// T bar
    t_bar: char,
}

impl Symbols {
    fn new() -> Self {
        Symbols {
            v_bar: '│',
            h_bar: '─',
            t_bar: '┬',
            l_cross: '├',
            r_cross: '┤',
            x_bar: '┼',
            l_bot: '╰',
            r_bot: '╯',
            l_top: '╭',
            r_top: '╮',
        }
    }
}

// draw tree view
pub struct TreeView<S: Display, T: Display> {
    pub data: S,
    pub children: Option<Vec<TreeView<T, S>>>,
}

pub trait TreeViewNode {
    fn draw(&self, max_depth: usize) -> String;
}

lazy_static! {
    static ref SYMBOLS: Symbols = Symbols::new();
}

#[inline]
fn draw_last_branch() -> String {
    SYMBOLS.l_bot.to_string() + &SYMBOLS.h_bar.to_string() + &draw_2_h()
}

#[inline]
fn draw_cross_branch() -> String {
    SYMBOLS.l_cross.to_string() + &SYMBOLS.h_bar.to_string() + &draw_2_h()
}

#[inline]
fn draw_2_h() -> String {
    SYMBOLS.h_bar.to_string() + &SYMBOLS.h_bar.to_string()
}

fn level_padding() -> String {
    SYMBOLS.v_bar.to_string() + "   "
}

fn empty_padding() -> String {
    " ".repeat(4)
}

impl<S, T> TreeView<S, T>
where
    S: Display,
    T: Display,
{
    fn new(data: S, children: Option<Vec<TreeView<T, S>>>) -> Self {
        TreeView { data, children }
    }
    fn draw_node(&self, cur_depth: usize, max_depth: usize, is_last: bool) -> Option<Vec<String>> {
        if cur_depth >= max_depth {
            return None;
        }
        assert!(cur_depth < max_depth);
        let mut lines = Vec::new();
        if is_last {
            lines.push(draw_last_branch() + " " + &self.data.to_string());
        } else {
            lines.push(draw_cross_branch() + " " + &self.data.to_string());
        }
        // children
        if let Some(ref nodes) = self.children {
            if !nodes.is_empty() {
                // sub tree
                for (i, node) in nodes.iter().enumerate() {
                    let sub = node.draw_node(cur_depth + 1, max_depth, i == nodes.len() - 1);
                    if let Some(mut sub_lines) = sub {
                        if cur_depth > 0 {
                            if !is_last {
                                lines.push(
                                    sub_lines
                                        .iter_mut()
                                        .map(|line| level_padding() + line)
                                        .collect::<Vec<String>>()
                                        .join("\n"),
                                );
                            } else {
                                lines.push(
                                    sub_lines
                                        .iter_mut()
                                        .map(|line| empty_padding() + line)
                                        .collect::<Vec<String>>()
                                        .join("\n"),
                                );
                            }
                        } else {
                            lines.push(sub_lines.join("\n"));
                        }
                    }
                }
            }
        }
        Some(lines)
    }
}

impl<S, T> TreeViewNode for TreeView<S, T>
where
    S: Display,
    T: Display,
{
    fn draw(&self, max_depth: usize) -> String {
        self.draw_node(0, max_depth, false).unwrap().join("\n")
    }
}

#[cfg(test)]
mod test_draw {
    use super::*;
    use super::{TreeView, TreeViewNode};
    struct Foo;
    impl Display for Foo {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            f.write_str("foo")
        }
    }
    #[test]
    fn test_foo() {
        let mut root = TreeView::new(Foo {}, None);
        let mut children = Vec::new();
        for _ in 0..6 {
            children.push(TreeView::new(Foo {}, None));
        }
        let mut children2 = Vec::new();
        for _ in 0..3 {
            children2.push(TreeView::new(Foo {}, None));
        }
        let mut children3 = Vec::new();
        for _ in 0..4 {
            children3.push(TreeView::new(Foo {}, None));
        }
        let mut children4 = Vec::new();
        for _ in 0..4 {
            children4.push(TreeView::new(Foo {}, None));
        }
        let mut children5 = Vec::new();
        for _ in 0..4 {
            children5.push(TreeView::new(Foo {}, None));
        }
        children3[1].children = Some(children4);
        children[1].children = Some(children2);
        children[3].children = Some(children3);
        children[5].children = Some(children5);
        root.children = Some(children);
        println!();
        println!("{}", root.draw(5));
    }
}
