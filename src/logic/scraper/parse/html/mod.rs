pub mod nodes;
pub mod properties;

use nodes::{child::Child, node::Node};

pub(crate) mod contains {
    pub fn attributes(header: &str) -> bool {
        header.contains('=')
    }
}

pub(crate) mod is {
    pub fn open(header: &str) -> bool {
        child(header)
            && header.starts_with('<')
            && !header.ends_with("/>")
            && !header.starts_with("</")
    }
    pub fn close(header: &str) -> bool {
        child(header) && header.starts_with("</")
    }

    pub fn comment(header: &str) -> bool {
        header.starts_with("<!--") && header.ends_with("-->")
    }
    pub fn declaration(header: &str) -> bool {
        header.starts_with("<?") && header.ends_with("?>")
    }
    pub fn child(header: &str) -> bool {
        !comment(header) && !declaration(header)
    }
}

pub struct HtmlParseOptions {
    pub(crate) trim_content: bool,
}
impl HtmlParseOptions {
    pub fn trim_content(&mut self, should_trim: bool) -> &mut Self {
        self.trim_content = should_trim;
        self
    }
}
impl Default for HtmlParseOptions {
    fn default() -> Self {
        Self { trim_content: true }
    }
}
pub struct Html {
    pub inner: String,
}

//Init
impl Html {
    pub fn from(html: String) -> Self {
        Self { inner: html }
    }
    fn next(&self, start_i: usize) -> Option<(usize, usize)> {
        let open_i = self.inner[start_i..].find('<')? + start_i;
        let close_i = self.inner[open_i..].find('>')? + open_i;
        Some((open_i, close_i))
    }
    fn is_valid_content(s: &str) -> bool {
        !s.contains('<') && !s.trim().is_empty()
    }
    pub fn parse(&self, options: Option<HtmlParseOptions>) -> Node {
        let options = options.unwrap_or_default();
        let mut last_open_i = 0;
        let mut last_close_i = 0;
        let mut stack = vec![Node::Child(Child::from_inner_html("document".to_owned()))];
        while let Some((open_i, close_i)) = self.next(last_open_i) {
            //let header: String = html.html.drain(open_i..close_i + 1).collect();
            let header = self.inner[open_i..close_i + 1].to_string();
            let is_open = is::open(&header);
            let is_close = is::close(&header);
            let node = Node::from_header(header);

            if is_open {
                stack.push(node);
            }
            else if is_close {
                let mut popped_node = stack.pop().unwrap();
                let content = &self.inner[last_close_i..open_i];
                if Self::is_valid_content(content) {
                    if options.trim_content {
                        popped_node.push_content(content.trim().to_string());
                    }
                    else {
                        popped_node.push_content(content.to_string());
                    }
                }

                //popped_node.push_content(content.clone());
                stack.last_mut().unwrap().push(popped_node);
            }
            else {
                stack.last_mut().unwrap().push(node);
            }
            last_open_i = open_i + 1;
            last_close_i = close_i + 1;
        }
        assert!(
            stack.len() == 1,
            "Length is {} when it should be 1",
            stack.len()
        );

        stack.pop().unwrap()
    }
}
