use std::collections::HashMap;

use tree_sitter::{Node, Parser, Tree};

use crate::models::{Attribute, FunctionInfo, HalCall, ParseError, Position, VariableInfo};

const CMSIS_QUALIFIERS: [(&str, &str); 4] = [
    ("__I", "const volatile"),
    ("__IO", "volatile"),
    ("__O", "volatile"),
    ("__VO", "volatile"),
];

pub struct Normalized {
    pub text: String,
    replacements: Vec<(usize, usize, usize)>,
}

impl Normalized {
    pub(crate) fn build(source: &str) -> Self {
        let bytes = source.as_bytes();
        let is_ident_char = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
        let mut text = String::with_capacity(source.len());
        let mut replacements = Vec::new();
        let mut i = 0;
        while i < source.len() {
            let matched = CMSIS_QUALIFIERS.iter().find_map(|(mac, kw)| {
                if source[i..].starts_with(mac)
                    && (i == 0 || !is_ident_char(bytes[i - 1]))
                    && (i + mac.len() == source.len() || !is_ident_char(bytes[i + mac.len()]))
                {
                    Some((*mac, *kw))
                } else {
                    None
                }
            });
            if let Some((mac, keyword)) = matched {
                replacements.push((i, mac.len(), keyword.len()));
                text.push_str(keyword);
                i += mac.len();
            } else {
                let ch_len = source[i..]
                    .chars()
                    .next()
                    .map(|c| c.len_utf8())
                    .unwrap_or(1);
                text.push_str(&source[i..i + ch_len]);
                i += ch_len;
            }
        }
        Self { text, replacements }
    }

    pub(crate) fn mod_to_orig(&self, pos: usize) -> usize {
        let mut delta: i64 = 0;
        for (o_start, o_len, m_len) in &self.replacements {
            let m_start = (*o_start as i64) + delta;
            let pos = pos as i64;
            if pos >= m_start + (*m_len as i64) {
                delta += (*o_len as i64) - (*m_len as i64);
            } else if pos >= m_start {
                return *o_start;
            } else {
                break;
            }
        }
        (pos as i64 + delta) as usize
    }
}

pub(crate) fn line_col_of(source: &str, byte: usize) -> (usize, usize) {
    let clamped = byte.min(source.len());
    let mut line = 1;
    let mut line_start = 0;
    for (i, b) in source.as_bytes()[..clamped].iter().enumerate() {
        if *b == b'\n' {
            line += 1;
            line_start = i + 1;
        }
    }
    let col = source[line_start..clamped].chars().count() + 1;
    (line, col)
}

fn position_of(node: Node, source: &str, normalized: &Normalized) -> Position {
    let range = node.byte_range();
    let byte_start = normalized.mod_to_orig(range.start);
    let byte_end = normalized.mod_to_orig(range.end);
    let (line, col) = line_col_of(source, byte_start);
    Position {
        byte_start,
        byte_end,
        line,
        col,
    }
}

fn node_text<'a>(node: Node<'a>, norm_text: &'a str) -> &'a str {
    let range = node.byte_range();
    &norm_text[range.start..range.end]
}

fn declarator_name(declarator_text: &str) -> String {
    declarator_text
        .split('(')
        .next()
        .unwrap_or("")
        .trim()
        .trim_end_matches('*')
        .trim()
        .to_string()
}

const HAL_PREFIXES: [&str; 2] = ["HAL_", "LL_"];

pub struct Analyzer {
    parser: Parser,
}

impl Analyzer {
    pub fn new() -> Self {
        let mut parser = Parser::new();
        parser
            .set_language(&tree_sitter_c::LANGUAGE.into())
            .expect("failed to load C grammar");
        Self { parser }
    }

    pub fn analyze(&mut self, source: &str) -> Analysis {
        let normalized = Normalized::build(source);
        let tree = match self.parser.parse(&normalized.text, None) {
            Some(t) => t,
            None => return Analysis::default(),
        };
        let mut walker = AstWalker::new(source, &normalized);
        walk_root(&tree, &mut walker);
        Analysis {
            functions: walker.functions,
            variables: walker.variables,
            hal_calls: walker.hal_calls,
            parse_errors: walker.parse_errors,
        }
    }
}

impl Default for Analyzer {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Default)]
pub struct Analysis {
    pub functions: Vec<FunctionInfo>,
    pub variables: Vec<VariableInfo>,
    pub hal_calls: Vec<HalCall>,
    pub parse_errors: Vec<ParseError>,
}

struct AstWalker<'a> {
    source: &'a str,
    norm: &'a Normalized,
    functions: Vec<FunctionInfo>,
    variables: Vec<VariableInfo>,
    hal_calls: Vec<HalCall>,
    parse_errors: Vec<ParseError>,
}

impl<'a> AstWalker<'a> {
    fn new(source: &'a str, norm: &'a Normalized) -> Self {
        Self {
            source,
            norm,
            functions: vec![],
            variables: vec![],
            hal_calls: vec![],
            parse_errors: vec![],
        }
    }

    fn text<'t>(&self, node: Node<'t>) -> &'t str
    where
        'a: 't,
    {
        node_text(node, &self.norm.text)
    }

    fn position(&self, node: Node) -> Position {
        position_of(node, self.source, self.norm)
    }

    fn collect_attributes(&self, node: Node) -> Vec<Attribute> {
        let mut attrs = Vec::new();
        let mut stack = vec![node];
        while let Some(current) = stack.pop() {
            if current.kind() == "attribute_specifier" {
                if let Some(attr) = self.parse_attribute_specifier(current) {
                    attrs.push(attr);
                }
            }
            let mut cursor = current.walk();
            for child in current.children(&mut cursor) {
                stack.push(child);
            }
        }
        attrs
    }

    fn parse_attribute_specifier(&self, node: Node) -> Option<Attribute> {
        let mut cursor = node.walk();
        let arg_list = node
            .children(&mut cursor)
            .find(|c| c.kind() == "argument_list")?;
        let mut inner = arg_list.walk();
        for part in arg_list.children(&mut inner) {
            match part.kind() {
                "call_expression" => {
                    let name = part
                        .child_by_field_name("function")
                        .map(|f| self.text(f).to_string());
                    if let Some(name) = name.filter(|n| !n.is_empty()) {
                        let args = part
                            .child_by_field_name("arguments")
                            .map(|a| {
                                self.text(a)
                                    .trim_start_matches('(')
                                    .trim_end_matches(')')
                                    .split(',')
                                    .map(|s| s.trim().trim_matches('"').to_string())
                                    .filter(|s| !s.is_empty())
                                    .collect()
                            })
                            .unwrap_or_default();
                        return Some(Attribute { name, args });
                    }
                }
                "identifier" => {
                    return Some(Attribute {
                        name: self.text(part).to_string(),
                        args: vec![],
                    });
                }
                _ => {}
            }
        }
        None
    }

    fn record_hal_calls(&mut self, body: Node) {
        let mut stack = vec![body];
        while let Some(node) = stack.pop() {
            if node.kind() == "call_expression" {
                if let Some(func) = node.child_by_field_name("function") {
                    let callee = self.text(func);
                    if HAL_PREFIXES.iter().any(|p| callee.starts_with(p)) {
                        self.hal_calls.push(HalCall {
                            callee: callee.to_string(),
                            position: self.position(func),
                        });
                    }
                }
            }
            let mut cursor = node.walk();
            for child in node.children(&mut cursor) {
                stack.push(child);
            }
        }
        self.hal_calls.sort_by_key(|c| c.position.byte_start);
    }
}

fn recover_error_contents(node: Node, walker: &mut AstWalker) {
    let mut cursor = node.walk();
    let children: Vec<Node> = node.children(&mut cursor).collect();
    if let Some(declarator) = children.iter().find(|c| c.kind() == "function_declarator") {
        let name = declarator_name(walker.text(*declarator));
        if !name.is_empty() {
            walker.functions.push(FunctionInfo {
                name,
                signature: walker.text(*declarator).to_string(),
                position: walker.position(*declarator),
                attributes: vec![],
            });
        }
    }
    for child in &children {
        if child.kind() != "function_definition" {
            walker.record_hal_calls(*child);
        }
    }
}

fn walk_root(tree: &Tree, walker: &mut AstWalker) {
    let mut stack = vec![tree.root_node()];
    while let Some(node) = stack.pop() {
        match node.kind() {
            "ERROR" => {
                walker.parse_errors.push(ParseError {
                    message: format!(
                        "unparseable construct at {}",
                        walker.text(node).chars().take(60).collect::<String>()
                    ),
                    position: walker.position(node),
                });
                recover_error_contents(node, walker);
            }
            "function_definition" => {
                let mut cursor = node.walk();
                let mut declarator_text = String::new();
                let mut return_type = String::new();
                for child in node.children(&mut cursor) {
                    match child.kind() {
                        "primitive_type"
                        | "type_identifier"
                        | "sized_type_specifier"
                        | "struct_specifier" => {
                            return_type = walker.text(child).to_string();
                        }
                        "function_declarator" | "pointer_declarator" => {
                            declarator_text = walker.text(child).to_string();
                        }
                        _ => {}
                    }
                }
                let name = declarator_name(&declarator_text);
                if !name.is_empty() {
                    walker.functions.push(FunctionInfo {
                        name,
                        signature: format!("{} {}", return_type, declarator_text)
                            .trim()
                            .to_string(),
                        position: walker.position(node),
                        attributes: walker.collect_attributes(node),
                    });
                }
                if let Some(body) = node.child_by_field_name("body") {
                    walker.record_hal_calls(body);
                }
            }
            "declaration" => {
                collect_declarations(node, walker);
            }
            _ => {}
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            stack.push(child);
        }
    }
}

fn collect_declarations(node: Node, walker: &mut AstWalker) {
    let mut cursor = node.walk();
    let mut type_text = String::new();
    let mut storage = String::new();
    let attrs: Vec<Attribute> = walker.collect_attributes(node);
    for child in node.children(&mut cursor) {
        match child.kind() {
            "storage_class_specifier" => storage = walker.text(child).to_string(),
            "primitive_type" | "type_identifier" | "sized_type_specifier" | "struct_specifier" => {
                type_text = walker.text(child).to_string()
            }
            _ => {}
        }
    }
    if type_text.is_empty() {
        return;
    }
    for child in node.children(&mut cursor) {
        if !matches!(
            child.kind(),
            "init_declarator"
                | "pointer_declarator"
                | "array_declarator"
                | "identifier"
                | "function_declarator"
        ) {
            continue;
        }
        let is_function_decl = matches!(child.kind(), "function_declarator")
            || (child.kind() == "init_declarator"
                && child.child(0).map(|d| d.kind()) == Some("function_declarator"));
        if is_function_decl {
            continue;
        }
        if child.kind() == "identifier" {
            walker.variables.push(VariableInfo {
                name: walker.text(child).to_string(),
                r#type: type_text.clone(),
                storage: storage.clone(),
                position: walker.position(child),
                attributes: attrs.clone(),
            });
            continue;
        }
        let declarator = match child.child_by_field_name("declarator") {
            Some(d) => d,
            None => continue,
        };
        {
            let mut name_text = walker.text(declarator);
            while let Some(stripped) = name_text.strip_prefix('*') {
                name_text = stripped.trim_start();
            }
            let name = name_text
                .split(['(', '[', '='])
                .next()
                .unwrap_or("")
                .trim()
                .to_string();
            if name.is_empty() || name.parse::<f64>().is_ok() {
                continue;
            }
            let effective_type = if walker.text(declarator).trim_start().starts_with('*') {
                format!("{}*", type_text)
            } else {
                type_text.clone()
            };
            let child_attrs = walker.collect_attributes(declarator);
            let mut merged = attrs.clone();
            merged.extend(child_attrs);
            walker.variables.push(VariableInfo {
                name,
                r#type: effective_type,
                storage: storage.clone(),
                position: walker.position(child),
                attributes: dedup_attributes(merged),
            });
        }
    }
}

fn dedup_attributes(attrs: Vec<Attribute>) -> Vec<Attribute> {
    let mut seen: HashMap<(String, Vec<String>), ()> = HashMap::new();
    attrs
        .into_iter()
        .filter(|a| seen.insert((a.name.clone(), a.args.clone()), ()).is_none())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn analyze_str(code: &str) -> Analysis {
        Analyzer::new().analyze(code)
    }

    #[test]
    fn extracts_simple_function() {
        let a = analyze_str("int add(int a, int b) { return a + b; }");
        assert_eq!(a.functions.len(), 1);
        let f = &a.functions[0];
        assert_eq!(f.name, "add");
        assert_eq!(f.signature, "int add(int a, int b)");
        assert_eq!(f.position.line, 1);
        assert_eq!(f.position.byte_start, 0);
        assert!(a.parse_errors.is_empty());
    }

    #[test]
    fn extracts_static_pointer_variable() {
        let a = analyze_str("static uint8_t *buf = 0;");
        assert_eq!(a.variables.len(), 1);
        let v = &a.variables[0];
        assert_eq!(v.name, "buf");
        assert_eq!(v.r#type, "uint8_t*");
        assert_eq!(v.storage, "static");
    }

    #[test]
    fn extracts_multiple_variables_from_one_declaration() {
        let a = analyze_str("int a, b;");
        assert_eq!(a.variables.len(), 2);
        assert_eq!(a.variables[0].name, "a");
        assert_eq!(a.variables[1].name, "b");
    }

    #[test]
    fn extracts_section_attribute() {
        let a = analyze_str("static uint8_t __attribute__((section(\".ccmram\"))) buf[64];");
        assert_eq!(a.variables.len(), 1);
        let v = &a.variables[0];
        assert_eq!(v.name, "buf");
        assert_eq!(v.attributes.len(), 1);
        assert_eq!(v.attributes[0].name, "section");
        assert_eq!(v.attributes[0].args, vec![".ccmram"]);
    }

    #[test]
    fn extracts_always_inline_style_from_ema_header() {
        let code = "__attribute__((always_inline)) static inline ufixed_t ema_update(EMA_Filter *f, float x)\n{\n    return x;\n}";
        let a = analyze_str(code);
        assert_eq!(a.functions.len(), 1);
        assert_eq!(a.functions[0].name, "ema_update");
        assert_eq!(a.functions[0].attributes.len(), 1);
        assert_eq!(a.functions[0].attributes[0].name, "always_inline");
    }

    #[test]
    fn function_declaration_is_not_a_variable() {
        let a = analyze_str("static HAL_StatusTypeDef shift_send(uint8_t *data, size_t len);");
        assert!(a.variables.is_empty());
        assert!(a.functions.is_empty());
    }

    #[test]
    fn extracts_hal_calls_with_positions() {
        let code = "void f(void) { HAL_GPIO_WritePin(GPIOA, 1); int x = HAL_GetTick(); }";
        let a = analyze_str(code);
        assert_eq!(a.hal_calls.len(), 2);
        assert_eq!(a.hal_calls[0].callee, "HAL_GPIO_WritePin");
        let pos = a.hal_calls[0].position;
        assert_eq!(&code[pos.byte_start..pos.byte_end], "HAL_GPIO_WritePin");
        assert_eq!(pos.line, 1);
        assert_eq!(a.hal_calls[1].callee, "HAL_GetTick");
    }

    #[test]
    fn non_hal_calls_are_ignored() {
        let a = analyze_str("void f(void) { memcpy(a, b, 3); printf(\"x\"); }");
        assert!(a.hal_calls.is_empty());
    }

    #[test]
    fn extracts_weak_attribute_without_args() {
        let a = analyze_str("__attribute__((weak)) void f(void) { return; }");
        assert_eq!(a.functions.len(), 1);
        assert_eq!(a.functions[0].name, "f");
        assert_eq!(a.functions[0].attributes.len(), 1);
        assert_eq!(a.functions[0].attributes[0].name, "weak");
        assert!(a.functions[0].attributes[0].args.is_empty());
    }

    #[test]
    fn parses_error_node_reported() {
        let a = analyze_str("int int int }{");
        assert!(!a.parse_errors.is_empty());
        assert!(a.parse_errors[0].position.line >= 1);
    }

    #[test]
    fn recovers_function_from_unbalanced_preprocessor_braces() {
        let code = "void f(void)\n{\n#if X\n  {\n#endif\n  HAL_IncTick();\n#if X\n  }\n#endif\n}";
        let a = analyze_str(code);
        assert!(a.functions.iter().any(|f| f.name == "f"));
        assert!(a.hal_calls.iter().any(|c| c.callee == "HAL_IncTick"));
    }

    #[test]
    fn cmsis_io_qualifier_parses_without_errors() {
        let code = "void f(void) { __IO uint32_t tmp = 0x00; register __IO uint32_t index; }";
        let a = analyze_str(code);
        assert!(a.parse_errors.is_empty(), "errors: {:?}", a.parse_errors);
        assert_eq!(a.variables.len(), 2);
        let names: Vec<&str> = a.variables.iter().map(|v| v.name.as_str()).collect();
        assert!(names.contains(&"tmp"));
        assert!(names.contains(&"index"));
        let tmp = a.variables.iter().find(|v| v.name == "tmp").unwrap();
        assert_eq!(tmp.r#type, "uint32_t");
    }

    #[test]
    fn cmsis_qualifier_positions_map_to_original_bytes() {
        let code = "__IO uint32_t flag;\nvoid f(void) { HAL_GPIO_WritePin(flag, 1); }";
        let a = analyze_str(code);
        assert!(a.parse_errors.is_empty());
        let pos = a.hal_calls[0].position;
        assert_eq!(&code[pos.byte_start..pos.byte_end], "HAL_GPIO_WritePin");
        assert_eq!(pos.line, 2);
        let flag = a.variables.first().unwrap();
        assert_eq!(
            &code[flag.position.byte_start..flag.position.byte_end],
            "flag"
        );
        assert_eq!(flag.position.line, 1);
    }

    #[test]
    fn position_line_and_byte_are_accurate() {
        let code = "int a;\n\nint foo(void) {\n  return 0;\n}\n";
        let a = analyze_str(code);
        let f = &a.functions[0];
        assert_eq!(f.position.line, 3);
        assert_eq!(f.position.byte_start, code.find("int foo").unwrap());
        assert_eq!(f.position.col, 1);
    }
}
