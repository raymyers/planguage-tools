use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentSummary {
    pub path: PathBuf,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SearchQuery {
    pub needle: String,
    pub path_prefix: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DocumentStats {
    pub markdown_files: usize,
    pub directories_with_markdown: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TreeNode {
    pub name: String,
    pub children: Vec<TreeNode>,
    pub is_document: bool,
}

impl TreeNode {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            children: Vec::new(),
            is_document: false,
        }
    }

    pub fn from_documents(documents: &[DocumentSummary]) -> Self {
        let mut root = Self::new(".");

        for document in documents {
            let mut current = &mut root;

            for segment in document
                .path
                .iter()
                .map(|part| part.to_string_lossy().into_owned())
            {
                let existing_index = current
                    .children
                    .iter()
                    .position(|child| child.name == segment);

                let index = match existing_index {
                    Some(index) => index,
                    None => {
                        current.children.push(Self::new(segment));
                        current.children.len() - 1
                    }
                };

                current = &mut current.children[index];
            }

            current.is_document = true;
        }

        sort_tree(&mut root);
        root
    }
}

fn sort_tree(node: &mut TreeNode) {
    for child in &mut node.children {
        sort_tree(child);
    }

    node.children
        .sort_by(|left, right| match (left.is_document, right.is_document) {
            (false, true) => std::cmp::Ordering::Less,
            (true, false) => std::cmp::Ordering::Greater,
            _ => left.name.cmp(&right.name),
        });
}
