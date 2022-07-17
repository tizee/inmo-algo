use lazy_static::lazy_static;

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
pub struct TreeView {
    pub data: String,
    pub children: Option<Vec<TreeView>>,
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
// TODO: style
pub struct TreeViewStyle {
    branch_offset: TreeViewOffset,
    node_left_padding: usize,
}
pub enum TreeViewOffset {
    Start,
    End,
}

impl Default for TreeViewStyle {
    fn default() -> Self {
        TreeViewStyle {
            branch_offset: TreeViewOffset::Start,
            node_left_padding: 2,
        }
    }
}

impl TreeView {
    pub fn new<T: AsRef<str>>(data: T, children: Option<Vec<TreeView>>) -> Self {
        let data = data.as_ref();
        TreeView { data:data.to_string(), children }
    }
    // render tree by lines
    fn draw_node(
        &self,
        cur_depth: usize,
        max_depth: usize,
        is_last: bool,
        style: &TreeViewStyle,
    ) -> Option<Vec<String>> {
        if cur_depth >= max_depth {
            return None;
        }
        assert!(cur_depth < max_depth);
        let mut lines = Vec::new();

        let padding = " ".repeat(style.node_left_padding);
        if cur_depth > 0 {
            if is_last {
                lines.push(draw_last_branch() + &padding + &self.data.to_string());
            } else {
                lines.push(draw_cross_branch() + &padding + &self.data.to_string());
            }
        } else {
            lines.push(self.data.to_string());
        }
        let root_offset = match style.branch_offset {
            TreeViewOffset::Start => "".to_string(),
            TreeViewOffset::End => " ".repeat(self.data.len()),
        };

        // children
        if let Some(ref nodes) = self.children {
            if !nodes.is_empty() {
                // sub tree
                for (i, node) in nodes.iter().enumerate() {
                    let sub = node.draw_node(cur_depth + 1, max_depth, i == nodes.len() - 1, style);
                    let node_offset = match style.branch_offset {
                        TreeViewOffset::Start => "".to_string(),
                        TreeViewOffset::End => " ".repeat(node.data.len()),
                    };
                    if let Some(sub_lines) = sub {
                        for line in sub_lines.iter() {
                            if cur_depth > 0 {
                                if !is_last {
                                    lines.push(level_padding() + &padding + &node_offset + line);
                                } else {
                                    lines.push(empty_padding() + &padding + &node_offset + line);
                                }
                            } else {
                                lines.push(root_offset.clone() + line);
                            }
                        }
                    }
                }
            }
        }
        Some(lines)
    }

    pub fn draw_default(&self, max_depth: usize) -> String {
        self.draw_node(0, max_depth, false, &TreeViewStyle::default())
            .unwrap()
            .join("\n")
    }
    pub fn draw_with_style(&self, max_depth: usize, style: TreeViewStyle) -> String {
        self.draw_node(0, max_depth, false, &style)
            .unwrap()
            .join("\n")
    }
}

#[cfg(test)]
mod test_draw {
    use super::TreeView;
    #[test]
    fn test_foo() {
        let mut root = TreeView::new("foo", None);
        let mut children = Vec::new();
        for _ in 0..6 {
            children.push(TreeView::new("foo", None));
        }
        let mut children2 = Vec::new();
        for _ in 0..3 {
            children2.push(TreeView::new("foo", None));
        }
        let mut children3 = Vec::new();
        for _ in 0..4 {
            children3.push(TreeView::new("foo", None));
        }
        let mut children4 = Vec::new();
        for _ in 0..4 {
            children4.push(TreeView::new("foo", None));
        }
        let mut children5 = Vec::new();
        for _ in 0..4 {
            children5.push(TreeView::new("foo", None));
        }
        children3[1].children = Some(children4);
        children[1].children = Some(children2);
        children[3].children = Some(children3);
        children[5].children = Some(children5);
        root.children = Some(children);
        println!();
        println!("{}", root.draw_default(5));
    }
}
