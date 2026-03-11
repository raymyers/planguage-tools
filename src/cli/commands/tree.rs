use std::process::ExitCode;

use crate::application::App;
use crate::cli::args::TreeArgs;
use crate::domain::document::TreeNode;

pub fn run(args: TreeArgs, app: &App) -> Result<ExitCode, crate::application::error::AppError> {
    let documents = app.list_documents()?;
    let filtered = documents
        .into_iter()
        .filter(|document| {
            args.path_prefix
                .as_ref()
                .is_none_or(|prefix| document.path.to_string_lossy().starts_with(prefix))
        })
        .collect::<Vec<_>>();

    let tree = TreeNode::from_documents(&filtered);

    for (index, child) in tree.children.iter().enumerate() {
        let is_last = index + 1 == tree.children.len();
        print_node(child, "", is_last);
    }

    Ok(ExitCode::SUCCESS)
}

fn print_node(node: &TreeNode, prefix: &str, is_last: bool) {
    let branch = if is_last { "└── " } else { "├── " };
    println!("{prefix}{branch}{}", node.name);

    let child_prefix = if is_last {
        format!("{prefix}    ")
    } else {
        format!("{prefix}│   ")
    };

    for (index, child) in node.children.iter().enumerate() {
        let is_last_child = index + 1 == node.children.len();
        print_node(child, &child_prefix, is_last_child);
    }
}
