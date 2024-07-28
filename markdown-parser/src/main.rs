use crate::lang::ProgrammingLanguage;

pub mod lang;

fn main() {
    let lang = ProgrammingLanguage::from_extension("R");
    println!("Found language {lang:?}");
}

pub enum MarkdownElement {
    PlainText(String),
    Header(usize, String),
    CodeBlock(ProgrammingLanguage, String),
    InlineCode(ProgrammingLanguage, String),
    List(ListType, Vec<String>),
    Link(String, String, Option<String>), // Added an optional title field
    Emphasis(EmphasisType, String),
    Image(String, String),
    Blockquote(String),
    HorizontalRule,
    Table(Vec<TableRow>, Option<TableRow>), // Added support for header row
}

pub enum ListType {
    Ordered,
    Unordered,
}

pub enum EmphasisType {
    Italic,
    Bold,
}

pub struct TableRow {
    pub cells: Vec<String>,
}

pub struct MarkdownDocument(Vec<MarkdownElement>);
