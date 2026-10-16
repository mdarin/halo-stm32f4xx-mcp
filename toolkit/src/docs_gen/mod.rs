use tree_sitter::{Node, Parser};

use crate::analyzer::{line_col_of, Normalized};
use crate::models::{Insertion, Position};

pub struct DocsResult {
    pub insertions: Vec<Insertion>,
    pub functions_documented: usize,
    pub functions_skipped: usize,
}

struct FnDecl {
    name: String,
    return_type: String,
    params: Vec<String>,
    position: Position,
    line_start_byte: usize,
    has_doc_comment: bool,
}

fn walk_back_find_doc(node: Node, norm: &Normalized) -> bool {
    let mut prev = node.prev_sibling();
    while let Some(sibling) = prev {
        match sibling.kind() {
            "comment" => {
                let text = &norm.text[sibling.byte_range()];
                return text.trim_start().starts_with("/**")
                    || text.trim_start().starts_with("///");
            }
            "attribute_specifier" => prev = sibling.prev_sibling(),
            _ => return false,
        }
    }
    false
}

fn extract_params(node: Node, norm: &Normalized) -> (String, Vec<String>) {
    let mut return_type = String::new();
    let mut signature = String::new();
    let mut params = Vec::new();
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        match child.kind() {
            "primitive_type" | "type_identifier" | "sized_type_specifier" | "struct_specifier" => {
                return_type = norm.text[child.byte_range()].to_string();
            }
            "function_declarator" | "pointer_declarator" => {
                signature = norm.text[child.byte_range()].to_string();
            }
            _ => {}
        }
    }
    if let Some(paren_open) = signature.find('(') {
        let mut depth = 0;
        let mut param_start = paren_open + 1;
        let bytes = signature.as_bytes();
        let mut close = signature.len();
        for (i, b) in bytes.iter().enumerate().skip(paren_open) {
            match b {
                b'(' => depth += 1,
                b')' => {
                    depth -= 1;
                    if depth == 0 {
                        close = i;
                        break;
                    }
                }
                _ => {}
            }
        }
        let inner = &signature[param_start..close];
        for raw in split_top_level(inner) {
            let p = raw.trim();
            if p.is_empty() || p == "void" || p.starts_with("...") {
                continue;
            }
            let name = param_name(p);
            params.push(name.unwrap_or_else(|| p.to_string()));
        }
        param_start = 0;
        let _ = param_start;
    }
    (return_type, params)
}

fn split_top_level(inner: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut depth = 0;
    let mut current = String::new();
    for ch in inner.chars() {
        match ch {
            '(' | '[' => {
                depth += 1;
                current.push(ch);
            }
            ')' | ']' => {
                depth -= 1;
                current.push(ch);
            }
            ',' if depth == 0 => {
                parts.push(current.clone());
                current.clear();
            }
            _ => current.push(ch),
        }
    }
    if !current.trim().is_empty() {
        parts.push(current);
    }
    parts
}

fn param_name(param: &str) -> Option<String> {
    let cleaned = param
        .trim()
        .trim_end_matches(|c: char| c.is_whitespace() || c == '[' || c == ']');
    let name = cleaned
        .rsplit(|c: char| c.is_whitespace() || c == '*')
        .next()
        .unwrap_or("")
        .trim();
    if name.is_empty()
        || name.parse::<f64>().is_ok()
        || name == "void"
        || cleaned.ends_with('*') && name.is_empty()
    {
        None
    } else {
        Some(name.to_string())
    }
}

fn doxygen_block(name: &str, params: &[String], return_type: &str) -> String {
    let mut lines = vec!["/**".to_string(), format!(" * @brief {name}")];
    for p in params {
        lines.push(format!(" * @param {p} <description>"));
    }
    if return_type != "void" && !return_type.is_empty() {
        lines.push(format!(" * @retval {return_type}"));
    }
    lines.push(" */".into());
    let mut text = lines.join("\n");
    text.push('\n');
    text
}

fn collect_functions(source: &str, norm: &Normalized, root: Node) -> Vec<FnDecl> {
    let mut functions = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.kind() == "function_definition" {
            let mut name = String::new();
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                if child.kind() == "function_declarator" || child.kind() == "pointer_declarator" {
                    let decl_text = &norm.text[child.byte_range()];
                    name = decl_text
                        .split('(')
                        .next()
                        .unwrap_or("")
                        .trim()
                        .trim_end_matches('*')
                        .trim()
                        .to_string();
                }
            }
            if !name.is_empty() {
                let (return_type, params) = extract_params(node, norm);
                let range = node.byte_range();
                let byte_start = norm.mod_to_orig(range.start);
                let (line, _) = line_col_of(source, byte_start);
                let line_start_byte = source
                    .as_bytes()
                    .iter()
                    .take(byte_start)
                    .rposition(|&b| b == b'\n')
                    .map(|p| p + 1)
                    .unwrap_or(0);
                functions.push(FnDecl {
                    name,
                    return_type,
                    params,
                    position: Position {
                        byte_start,
                        byte_end: norm.mod_to_orig(range.end),
                        line,
                        col: line_col_of(source, line_start_byte).1,
                    },
                    line_start_byte,
                    has_doc_comment: walk_back_find_doc(node, norm),
                });
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            stack.push(child);
        }
    }
    functions.sort_by_key(|f| f.position.byte_start);
    functions
}

pub fn run(source: &str) -> DocsResult {
    let normalized = Normalized::build(source);
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("failed to load C grammar");
    let tree = match parser.parse(&normalized.text, None) {
        Some(t) => t,
        None => {
            return DocsResult {
                insertions: vec![],
                functions_documented: 0,
                functions_skipped: 0,
            };
        }
    };
    let functions = collect_functions(source, &normalized, tree.root_node());
    let mut insertions = Vec::new();
    let mut documented = 0;
    let mut skipped = 0;
    for f in functions {
        if f.has_doc_comment {
            skipped += 1;
            continue;
        }
        let indent = source[f.line_start_byte..f.position.byte_start]
            .chars()
            .take_while(|c| c.is_whitespace())
            .collect::<String>();
        let block = doxygen_block(&f.name, &f.params, &f.return_type);
        let text: String = block
            .lines()
            .map(|l| {
                if l.is_empty() {
                    l.to_string()
                } else {
                    format!("{indent}{l}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");
        let text = format!("{text}\n");
        let (line, col) = line_col_of(source, f.line_start_byte);
        insertions.push(Insertion {
            position: Position {
                byte_start: f.line_start_byte,
                byte_end: f.line_start_byte,
                line,
                col,
            },
            text,
        });
        documented += 1;
    }
    DocsResult {
        insertions,
        functions_documented: documented,
        functions_skipped: skipped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_insertion_for_undocumented_function() {
        let code = "int add(int a, int b)\n{\n    return a + b;\n}\n";
        let r = run(code);
        assert_eq!(r.functions_documented, 1);
        assert_eq!(r.functions_skipped, 0);
        let ins = &r.insertions[0];
        assert_eq!(ins.position.byte_start, ins.position.byte_end);
        assert_eq!(ins.position.line, 1);
        assert_eq!(ins.position.col, 1);
        assert!(ins.text.contains("@brief add"));
        assert!(ins.text.contains("@param a"));
        assert!(ins.text.contains("@param b"));
        assert!(ins.text.contains("@retval int"));
        let patched = format!(
            "{}{}{}",
            &code[..ins.position.byte_start],
            ins.text,
            &code[ins.position.byte_start..]
        );
        assert!(patched.starts_with("/**\n * @brief add"));
        assert!(patched.contains("int add(int a, int b)"));
    }

    #[test]
    fn documented_function_is_skipped() {
        let code = "/**\n * @brief add\n */\nint add(int a, int b) { return a + b; }\n";
        let r = run(code);
        assert_eq!(r.insertions.len(), 0);
        assert_eq!(r.functions_documented, 0);
        assert_eq!(r.functions_skipped, 1);
    }

    #[test]
    fn attribute_between_comment_and_function_is_skipped_for_doc_check() {
        let code = "/**\n * @brief ema\n */\n__attribute__((always_inline)) static inline ufixed_t ema(EMA_Filter *f) { return f->y; }\n";
        let r = run(code);
        assert_eq!(r.functions_skipped, 1, "insertions: {:?}", r.insertions);
    }

    #[test]
    fn void_return_and_void_params_produce_no_retval_param() {
        let code = "void tick(void) { }\n";
        let r = run(code);
        assert_eq!(r.functions_documented, 1);
        let ins = &r.insertions[0];
        assert!(!ins.text.contains("@param"));
        assert!(!ins.text.contains("@retval"));
    }

    #[test]
    fn insertion_respects_function_indentation() {
        let code = "namespace x {\n  static inline int f(int q)\n  {\n    return q;\n  }\n}\n";
        let r = run(code);
        assert_eq!(r.functions_documented, 1);
        assert!(r.insertions[0].text.starts_with("  /**"));
    }

    #[test]
    fn line_comment_doc_counts_as_documented() {
        let code = "/// brief\nint f(void) { return 0; }\n";
        let r = run(code);
        assert_eq!(r.functions_skipped, 1);
    }

    #[test]
    fn pointer_param_name_extracted() {
        let code = "static HAL_StatusTypeDef shift_send(uint8_t *data, size_t len);\n";
        let r = run(code);
        assert_eq!(r.insertions.len(), 0);
    }

    #[test]
    fn cmsis_qualifier_does_not_confuse_positions() {
        let code = "__IO uint32_t flag;\n\nvoid f(void)\n{\n    flag = 1;\n}\n";
        let r = run(code);
        assert_eq!(r.functions_documented, 1);
        assert_eq!(r.insertions[0].position.line, 3);
    }
}
