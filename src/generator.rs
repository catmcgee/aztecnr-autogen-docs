use crate::parser::NoirFile;
use std::fs;
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use regex::Regex;


pub struct DocusaurusDoc {
    pub content: String,
    pub path: PathBuf,
}

pub enum SidebarItem {
    Category { label: String, items: Vec<SidebarItem> },
    Doc { id: String, label: String },
}

struct Library {
    name: String,
    files: Vec<NoirFile>,
}

fn parse_doc_comment(doc_comment: &str) -> (String, Vec<(String, String, String)>) {
    let mut description = String::new();
    let mut params = Vec::new();

    let param_regex = Regex::new(r"@param\s+(\w+)\s+(.+)").unwrap();

    for line in doc_comment.lines() {
        if let Some(captures) = param_regex.captures(line) {
            let param_name = captures.get(1).unwrap().as_str().to_string();
            let param_description = captures.get(2).unwrap().as_str().to_string();
            params.push((param_name, String::new(), param_description));
        } else {
            description.push_str(line);
            description.push('\n');
        }
    }

    (description.trim().to_string(), params)
}

pub fn generate_docusaurus_docs(input_dir: &str) -> (Vec<DocusaurusDoc>, Vec<SidebarItem>) {
    let mut docs = Vec::new();
    let mut libraries = HashMap::new();

    // Parse all Noir files
    for entry in fs::read_dir(input_dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("nr") {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains("// typedoc: true") {
                    let file_name = path.file_stem().unwrap().to_str().unwrap().to_string();
                    let noir_file = crate::parser::parse_noir_file(path.to_str().unwrap()).unwrap();
                    libraries.insert(file_name.clone(), Library { name: file_name, files: vec![noir_file] });
                }
            }
        }
    }

    // Generate main overview page
    docs.push(DocusaurusDoc {
        content: generate_main_overview(&libraries),
        path: PathBuf::from("aztec-nr.md"),
    });

    let mut sidebar = vec![SidebarItem::Doc {
        id: "aztec-nr".to_string(),
        label: "Aztec.nr Overview".to_string(),
    }];

    // Generate docs for each library (file in this case)
    for (name, library) in libraries {
        docs.push(DocusaurusDoc {
            content: generate_library_doc(&library),
            path: PathBuf::from(format!("{}.md", name)),
        });
        sidebar.push(SidebarItem::Doc {
            id: name.clone(),
            label: name,
        });
    }

    (docs, sidebar)
}
fn parse_directory(dir: &Path, files: &mut Vec<NoirFile>) {
    for entry in fs::read_dir(dir).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("nr") {
            if let Ok(content) = fs::read_to_string(&path) {
                if content.contains("// typedoc: true") {
                    let noir_file = crate::parser::parse_noir_file(path.to_str().unwrap()).unwrap();
                    files.push(noir_file);
                }
            }
        } else if path.is_dir() {
            parse_directory(&path, files);
        }
    }
}

fn generate_main_overview(libraries: &HashMap<String, Library>) -> String {
    let mut content = String::from("# Aztec.nr Project\n\n");
    content.push_str("Welcome to the Aztec.nr project documentation. This project consists of the following libraries:\n\n");
    
    for name in libraries.keys() {
        content.push_str(&format!("- [{}]({})\n", name, name));
    }

    content
}

fn generate_library_doc(library: &Library) -> String {
    let mut content = String::from(&format!("# {} Library\n\n", library.name));
    
    if let Some(file) = library.files.first() {
        content.push_str(&generate_file_content(file));
    }

    content
}

fn generate_aztec_docs(aztec_library: &Library) -> (Vec<DocusaurusDoc>, Vec<SidebarItem>) {
    let mut docs = Vec::new();
    let mut sidebar_items = Vec::new();

    // Generate main Aztec library page
    docs.push(DocusaurusDoc {
        content: generate_aztec_overview(aztec_library),
        path: PathBuf::from("aztec/index.md"),
    });
    sidebar_items.push(SidebarItem::Doc {
        id: "aztec/index".to_string(),
        label: "Aztec Overview".to_string(),
    });

    // Generate pages for each component
    for file in &aztec_library.files {
        let file_path = format!("aztec/{}.md", file.name);
        docs.push(DocusaurusDoc {
            content: generate_file_doc(file),
            path: PathBuf::from(&file_path),
        });
        sidebar_items.push(SidebarItem::Doc {
            id: file_path,
            label: file.name.clone(),
        });
    }

    (docs, sidebar_items)
}

fn generate_aztec_overview(aztec_library: &Library) -> String {
    let mut content = String::from("# Aztec Library\n\n");
    content.push_str("The Aztec library is the core component of the Aztec.nr project. It contains the following modules:\n\n");
    
    for file in &aztec_library.files {
        content.push_str(&format!("- [{}]({})\n", file.name, file.name));
    }

    content
}

fn generate_file_doc(file: &NoirFile) -> String {
    let mut content = String::from(&format!("# {}\n\n", file.name));
    content.push_str(&generate_file_content(file));
    content
}

fn generate_file_content(file: &NoirFile) -> String {
    let mut content = String::new();
    
    // Add file-level description
    content.push_str(&format!("# {} Module\n\n", file.name));
    content.push_str("This module contains the following components:\n\n");
    
    // Generate table of contents
    content.push_str("## Table of Contents\n");
    if !file.structs.is_empty() { content.push_str("- [Structs](#structs)\n"); }
    if !file.traits.is_empty() { content.push_str("- [Traits](#traits)\n"); }
    if !file.functions.is_empty() { content.push_str("- [Functions](#functions)\n"); }
    if !file.impls.is_empty() { content.push_str("- [Implementations](#implementations)\n"); }
    content.push_str("\n");

    // Generate struct documentation
    if !file.structs.is_empty() {
        content.push_str("## Structs\n\n");
        for struct_item in &file.structs {
            content.push_str(&format!("### {}\n\n", struct_item.name));
            // Add struct description, generic parameters, etc.
            content.push_str("Fields:\n");
            for field in &struct_item.fields {
                content.push_str(&format!("- `{}`: {}\n", field.name, field.ty));
            }
            content.push_str("\n");
        }
    }

    // Generate trait documentation
    if !file.traits.is_empty() {
        content.push_str("## Traits\n\n");
        for trait_item in &file.traits {
            content.push_str(&format!("### {}\n\n", trait_item.name));
            for method in &trait_item.methods {
                content.push_str(&format!("#### `{}`\n\n", method.name));
                if let Some(doc_comment) = &method.doc_comment {
                    content.push_str(&format!("{}\n\n", doc_comment));
                }
                content.push_str("```rust\n");
                content.push_str(&format!("fn {}(", method.name));
                // Add parameters
                content.push_str(")\n");
                if let Some(return_type) = &method.return_type {
                    content.push_str(&format!(" -> {}", return_type));
                }
                content.push_str("\n```\n\n");
            }
        }
    }

    // Generate function documentation
    if !file.functions.is_empty() {
        content.push_str("## Functions\n\n");
        for function in &file.functions {
            content.push_str(&format!("### `{}`\n\n", function.name));
            if let Some(doc_comment) = &function.doc_comment {
                content.push_str(&format!("{}\n\n", doc_comment));
            }
            content.push_str("```rust\n");
            content.push_str(&format!("fn {}(", function.name));
            // Add parameters
            content.push_str(")\n");
            if let Some(return_type) = &function.return_type {
                content.push_str(&format!(" -> {}", return_type));
            }
            content.push_str("\n```\n\n");
        }
    }

    // Generate impl documentation
    if !file.impls.is_empty() {
        content.push_str("## Implementations\n\n");
        for impl_item in &file.impls {
            content.push_str(&format!("### Impl for {}\n\n", impl_item.target));
            for method in &impl_item.methods {
                content.push_str(&format!("#### `{}`\n\n", method.name));
                if let Some(doc_comment) = &method.doc_comment {
                    let (description, params) = parse_doc_comment(doc_comment);
                    content.push_str(&format!("{}\n\n", description));

                    // Generate parameter table
                    if !params.is_empty() {
                        content.push_str("| Parameter | Type | Description |\n");
                        content.push_str("|-----------|------|-------------|\n");
                        for (name, _, desc) in params {
                            let param_type = method.params.iter()
                                .find(|p| p.name == name)
                                .map(|p| p.ty.clone())
                                .unwrap_or_else(|| "Unknown".to_string());
                            content.push_str(&format!("| `{}` | `{}` | {} |\n", name, param_type, desc));
                        }
                        content.push_str("\n");
                    }
                }
                content.push_str("```rust\n");
                content.push_str(&format!("fn {}(", method.name));
                // Add parameters
                let params: Vec<String> = method.params.iter()
                    .map(|p| format!("{}: {}", p.name, p.ty))
                    .collect();
                content.push_str(&params.join(", "));
                content.push_str(")");
                if let Some(return_type) = &method.return_type {
                    content.push_str(&format!(" -> {}", return_type));
                }
                content.push_str("\n```\n\n");
            }
        }
    }

    content

}

pub fn write_docusaurus_docs(docs: Vec<DocusaurusDoc>, sidebar: Vec<SidebarItem>, output_dir: &str) -> std::io::Result<()> {
    let docs_dir = Path::new(output_dir).join("docs");
    fs::create_dir_all(&docs_dir)?;

    for doc in docs {
        let file_path = docs_dir.join(doc.path);
        if let Some(parent) = file_path.parent() {
            fs::create_dir_all(parent)?;
        }
        fs::write(file_path, doc.content)?;
    }

    // Generate sidebar.js
    let sidebar_content = generate_sidebar_js(&sidebar);
    let sidebar_path = Path::new(output_dir).join("sidebars.js");
    fs::write(sidebar_path, sidebar_content)?;

    Ok(())
}

fn generate_sidebar_js(sidebar: &[SidebarItem]) -> String {
    let mut content = String::from("module.exports = {\n  someSidebar: [\n");

    for item in sidebar {
        content.push_str(&format_sidebar_item(item, 4));
    }

    content.push_str("  ],\n};\n");
    content
}

fn format_sidebar_item(item: &SidebarItem, indent: usize) -> String {
    let spaces = " ".repeat(indent);
    match item {
        SidebarItem::Category { label, items } => {
            let mut content = format!("{}{{type: 'category', label: '{}', items: [\n", spaces, label);
            for sub_item in items {
                content.push_str(&format_sidebar_item(sub_item, indent + 2));
            }
            content.push_str(&format!("{}]}},\n", spaces));
            content
        }
        SidebarItem::Doc { id, label } => {
            format!("{}{{type: 'doc', id: '{}', label: '{}'}},\n", spaces, id, label)
        }
    }
}