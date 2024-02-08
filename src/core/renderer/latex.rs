use std::path::PathBuf;

use crate::{RenderContext, Renderer};

pub struct LatexRenderer<'a> {
    pub render_context: RenderContext<'a>,
}

impl<'a> LatexRenderer<'a> {
    pub fn new(render_context: RenderContext<'a>) -> Self {
        Self { render_context }
    }
    fn escape(&self, text: &str) -> String {
        text.replace('&', "\\&")
            .replace('%', "\\%")
            .replace('$', "\\$")
            .replace('#', "\\#")
            .replace('_', "\\_")
            .replace('{', "\\{")
            .replace('}', "\\}")
            .replace('~', "\\~")
            .replace('^', "\\^")
    }

    pub fn render_table_header_row(&self, row: &markdown::mdast::Node) -> crate::Result<String> {
        let mut out = String::new();
        if let Some(children) = row.children() {
            for child in children {
                out.push_str(&format!(
                    "\\textbf{{{}}} & ",
                    self.render_node(child)?
                        .trim_matches(|c| c == ' ' || c == '&'),
                ));
            }
        }
        out = out.trim_matches(|c| c == ' ' || c == '&').to_string();
        out.push_str(" \\\\\n");
        out.push_str("\\hline\\vspace{2pt}\n");
        Ok(out)
    }
}

impl<'a> Renderer for LatexRenderer<'a> {
    fn get_context(&self) -> &RenderContext {
        &self.render_context
    }

    fn finalize_render(&self, data: crate::DataContext) -> crate::Result<String> {
        Ok(data.body)
    }

    fn render_bold(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str(&self.render_nodes(children)?);
        Ok(format!("\\textbf{{{}}}", out))
    }

    fn render_emphasis(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str(&self.render_nodes(children)?);
        Ok(format!("\\textit{{{}}}", out))
    }

    fn render_delete(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str(&self.render_nodes(children)?);
        Ok(format!("\\st{{{}}}", out))
    }

    fn render_thematic_break(&self) -> crate::Result<String> {
        Ok("{\\color{rulecolor}\\vspace{8pt}\\par\\noindent\\rule{\\textwidth}{0.4pt}\\vspace{8pt}}\n".to_string())
    }

    fn render_text(&self, text: &str) -> crate::Result<String> {
        Ok(self.escape(text))
    }

    fn render_list_item(
        &self,
        checked: Option<bool>,
        children: &[markdown::mdast::Node],
    ) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str(self.render_nodes(children)?.trim());
        Ok(format!("\\item {}\n", out))
    }

    fn render_jsx_element(
        &self,
        name: &str,
        attrs: std::collections::HashMap<String, String>,
        children: &[markdown::mdast::Node],
    ) -> crate::Result<String> {
        match name {
            "Field" => Ok(format!(
                "\\field{{{}}}{{{}}}{{{}}}{{\n{}\n}}\n",
                attrs
                    .get("name")
                    .map(|s| self.escape(s))
                    .unwrap_or_default(),
                attrs
                    .get("type")
                    .map(|s| self.escape(s))
                    .unwrap_or_default(),
                attrs
                    .get("type_link")
                    .map(|s| self.escape(s))
                    .unwrap_or_default(),
                self.render_nodes(children)?.trim()
            )),
            _ => Ok(String::new()),
        }
    }

    fn render_inline_code(&self, code: &str) -> crate::Result<String> {
        Ok(format!(
            "\\textbf{{\\color{{magenta}}{}}}",
            self.escape(code)
        ))
    }

    fn render_list(
        &self,
        ordered: bool,
        children: &[markdown::mdast::Node],
    ) -> crate::Result<String> {
        let mut out = String::new();
        let list_type = if ordered { "enumerate" } else { "itemize" };
        out.push_str(format!("\\begin{{{}}}\n", list_type).as_str());
        for child in children {
            out.push_str(&self.render_node(child)?);
        }
        out.push_str(format!("\\end{{{}}}\n", list_type).as_str());
        Ok(out)
    }

    fn render_code(
        &self,
        code: &str,
        lang: Option<String>,
        filepath: Option<PathBuf>,
    ) -> crate::Result<String> {
        Ok(format!(
            "\\vspace{{8pt}}\\begin{{lstlisting}}[]\n{}\n\\end{{lstlisting}}\\vspace{{3pt}}\n",
            code
        ))
    }

    fn handle_table(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut col_count = 0;
        for child in children {
            if let markdown::mdast::Node::TableRow(row) = child {
                col_count = row.children.len();
                break;
            }
        }

        let mut out = String::new();
        out.push_str(&format!(
            "\\begin{{tabular}}{{{}}}\n",
            "l ".repeat(col_count).trim()
        ));
        let mut i = children.iter();
        let Some(header_row) = i.next() else {
            return Ok(out);
        };
        out.push_str(&self.render_table_header_row(header_row)?);
        out.push_str(&self.render_nodes(i.as_slice())?);
        out.push_str("\\end{tabular}\n");
        Ok(out)
    }

    fn render_table_row(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str("");
        out.push_str(self.render_nodes(children)?.trim().trim_matches('&'));
        out.push_str("\\\\\n");
        Ok(out)
    }

    fn render_table_cell(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        out.push_str("");
        out.push_str(&self.render_nodes(children)?);
        out.push_str(" & ");
        Ok(out)
    }

    fn render_link(
        &self,
        url: &str,
        title: Option<String>,
        children: &[markdown::mdast::Node],
    ) -> crate::Result<String> {
        let mut out = String::new();
        for child in children {
            out.push_str(&self.render_node(child)?);
        }

        if url.starts_with('/') {
            Ok(format!(
                "\\hyperref[sec:{}]{{{}}}",
                url.trim_matches('/').replace(['/', '_', '#'], "-"),
                out
            ))
        } else {
            Ok(format!("\\href{{{}}}{{{}}}", url, out))
        }
    }

    fn render_heading(
        &self,
        depth: u8,
        children: &[markdown::mdast::Node],
    ) -> crate::Result<String> {
        let mut out = String::new();
        for child in children {
            out.push_str(&self.render_node(child)?);
        }

        let text = self.get_text(children);
        let slug = self.slug(&text.unwrap_or_default());
        let url = format!("{}/{}", self.render_context.document.url, slug)
            .trim_matches('/')
            .replace(['/', '#', '_'], "-");

        Ok(match depth {
            1 => format!("\\subsection{{{}}}\\label{{sec:{}}}\n", out, url),
            2 => format!("\\subsubsection*{{{}}}\\label{{sec:{}}}\n", out, url),
            _ => format!("\\subsubsection*{{{}}}\n\n", out),
        })
    }

    fn render_paragraph(&self, children: &[markdown::mdast::Node]) -> crate::Result<String> {
        let mut out = String::new();
        for child in children {
            out.push_str(&self.render_node(child)?);
        }
        Ok(format!("{}\n\n", out))
    }
}
