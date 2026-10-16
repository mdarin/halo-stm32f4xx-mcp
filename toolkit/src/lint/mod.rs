use tree_sitter::{Node, Parser};

use crate::analyzer::{line_col_of, Analyzer, Normalized};
use crate::models::{Flow, FlowUse, Position, Summary, VariableInfo, Violation};

pub struct LintResult {
    pub violations: Vec<Violation>,
    pub flows: Vec<Flow>,
}

struct Call {
    callee: String,
    callee_position: Position,
    arguments_text: String,
}

struct CallCollector<'a> {
    source: &'a str,
    norm: &'a Normalized,
    calls: Vec<Call>,
}

impl CallCollector<'_> {
    fn text(&self, node: Node) -> &str {
        &self.norm.text[node.byte_range()]
    }

    fn position(&self, node: Node) -> Position {
        let range = node.byte_range();
        let byte_start = self.norm.mod_to_orig(range.start);
        let byte_end = self.norm.mod_to_orig(range.end);
        let (line, col) = line_col_of(self.source, byte_start);
        Position {
            byte_start,
            byte_end,
            line,
            col,
        }
    }
}

fn collect_calls(node: Node, collector: &mut CallCollector) {
    if node.kind() == "call_expression" {
        if let Some(func) = node.child_by_field_name("function") {
            let callee = collector.text(func).to_string();
            let callee_position = collector.position(func);
            let arguments_text = node
                .child_by_field_name("arguments")
                .map(|a| collector.text(a).to_string())
                .unwrap_or_default();
            collector.calls.push(Call {
                callee,
                callee_position,
                arguments_text,
            });
        }
    }
    let mut cursor = node.walk();
    for child in node.children(&mut cursor) {
        collect_calls(child, collector);
    }
}

fn has_dma_in_callee(callee: &str) -> bool {
    callee.to_uppercase().contains("DMA")
}

fn ident_uses_in_dma_calls(
    root: Node,
    source: &str,
    norm: &Normalized,
    var_name: &str,
) -> Vec<Position> {
    let mut uses = Vec::new();
    let mut stack = vec![root];
    while let Some(node) = stack.pop() {
        if node.kind() == "call_expression" {
            let is_dma = node
                .child_by_field_name("function")
                .map(|f| has_dma_in_callee(&norm.text[f.byte_range()]))
                .unwrap_or(false);
            if is_dma {
                if let Some(args) = node.child_by_field_name("arguments") {
                    let mut arg_stack = vec![args];
                    while let Some(a) = arg_stack.pop() {
                        if a.kind() == "identifier" && &norm.text[a.byte_range()] == var_name {
                            let range = a.byte_range();
                            let byte_start = norm.mod_to_orig(range.start);
                            let byte_end = norm.mod_to_orig(range.end);
                            let (line, col) = line_col_of(source, byte_start);
                            uses.push(Position {
                                byte_start,
                                byte_end,
                                line,
                                col,
                            });
                        }
                        let mut c = a.walk();
                        for ch in a.children(&mut c) {
                            arg_stack.push(ch);
                        }
                    }
                }
            }
        }
        let mut cursor = node.walk();
        for child in node.children(&mut cursor) {
            stack.push(child);
        }
    }
    uses
}

fn is_ccm_variable(variable: &VariableInfo) -> bool {
    variable
        .attributes
        .iter()
        .filter(|a| a.name == "section")
        .any(|a| a.args.iter().any(|s| s.to_lowercase().contains("ccm")))
}

pub fn run(source: &str, flow: bool) -> LintResult {
    let normalized = Normalized::build(source);
    let mut parser = Parser::new();
    parser
        .set_language(&tree_sitter_c::LANGUAGE.into())
        .expect("failed to load C grammar");
    let tree = match parser.parse(&normalized.text, None) {
        Some(t) => t,
        None => {
            return LintResult {
                violations: vec![],
                flows: vec![],
            };
        }
    };
    let root = tree.root_node();

    let analysis = Analyzer::new().analyze(source);

    let mut collector = CallCollector {
        source,
        norm: &normalized,
        calls: vec![],
    };
    collect_calls(root, &mut collector);

    let mut violations = Vec::new();

    // Rule 1: DMA buffer in CCM RAM
    for var in &analysis.variables {
        if !is_ccm_variable(var) {
            continue;
        }
        let dma_uses = ident_uses_in_dma_calls(root, source, &normalized, &var.name);
        let name_hints_dma = var.name.to_lowercase().contains("dma");
        if dma_uses.is_empty() && !name_hints_dma {
            continue;
        }
        violations.push(Violation {
            rule: "dma_buffer_in_ccmram".into(),
            severity: "error".into(),
            message: format!(
                "DMA buffer '{}' placed in CCM RAM: DMA1/DMA2 cannot access CCM RAM",
                var.name
            ),
            position: var.position,
            hint: "Move buffer to .axisram / .sram1-3, or use DMA2D/MDMA".into(),
        });
    }

    // Rule 2: NVIC priority grouping != 4
    for call in &collector.calls {
        if call.callee != "HAL_NVIC_SetPriorityGrouping" {
            continue;
        }
        let args = call.arguments_text.trim();
        let args = args.strip_prefix('(').unwrap_or(args);
        let args = args.strip_suffix(')').unwrap_or(args);
        let arg = args.split(',').next().unwrap_or("").trim();
        if arg.starts_with("NVIC_PRIORITYGROUP_") && arg != "NVIC_PRIORITYGROUP_4" {
            violations.push(Violation {
                rule: "nvic_priority_grouping".into(),
                severity: "warning".into(),
                message: format!(
                    "NVIC priority grouping changed from default GROUP_4 to {arg}. This may cause interrupt conflicts if other code expects GROUP_4."
                ),
                position: call.callee_position,
                hint: "Use HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4) or ensure all code uses the same grouping consistently.".into(),
            });
        }
    }

    // Rule 3: USB OTG requires 48 MHz PLL48CLK
    let usb_calls: Vec<&Call> = collector
        .calls
        .iter()
        .filter(|c| c.callee == "HAL_PCD_Init" || c.callee == "HAL_HCD_Init")
        .collect();
    if !usb_calls.is_empty() && !source.contains("PLL48CLK") && !source.contains("PLLQ") {
        violations.push(Violation {
            rule: "usb_48mhz_required".into(),
            severity: "error".into(),
            message: "USB OTG initialized but PLL48CLK configuration not found. USB OTG requires exactly 48 MHz clock. Verify RCC PLL configuration.".into(),
            position: usb_calls[0].callee_position,
            hint: "Ensure PLL48CLK = (HSE * PLLN / PLLM) / PLLQ = 48 MHz. Check CubeMX Clock Configuration tab.".into(),
        });
    }

    // Rule 4: Flash ROP Level 2 (irreversible)
    for needle in ["FLASH_OB_RDP_LEVEL_2", "OB_RDP_LEVEL_2"] {
        if let Some(byte) = source.find(needle) {
            let (line, col) = line_col_of(source, byte);
            violations.push(Violation {
                rule: "flash_rop_level2".into(),
                severity: "error".into(),
                message: "ROP Level 2 is being set. This is IRREVERSIBLE and will permanently lock the chip! Debug access will be disabled forever.".into(),
                position: Position { byte_start: byte, byte_end: byte + needle.len(), line, col },
                hint: "Use ROP Level 1 (reversible, erases Flash) or remove ROP configuration entirely.".into(),
            });
            break;
        }
    }

    let mut flows = Vec::new();
    if flow {
        for var in &analysis.variables {
            let is_candidate = is_ccm_variable(var) || var.name.to_lowercase().contains("dma");
            if !is_candidate {
                continue;
            }
            let mut uses = vec![FlowUse {
                kind: "declaration".into(),
                position: var.position,
            }];
            let mut reaches_dma = false;
            for pos in ident_uses_in_dma_calls(root, source, &normalized, &var.name) {
                reaches_dma = true;
                uses.push(FlowUse {
                    kind: "dma_call".into(),
                    position: pos,
                });
            }
            flows.push(Flow {
                variable: var.name.clone(),
                uses,
                reaches_dma,
            });
        }
    }

    violations.sort_by_key(|v| v.position.byte_start);
    LintResult { violations, flows }
}

pub fn summarize(violations: &[Violation]) -> Summary {
    let mut summary = Summary::default();
    for v in violations {
        match v.severity.as_str() {
            "error" => summary.errors += 1,
            "warning" => summary.warnings += 1,
            _ => summary.infos += 1,
        }
    }
    summary
}

#[cfg(test)]
mod tests {
    use super::*;

    fn violations(code: &str) -> Vec<Violation> {
        run(code, false).violations
    }

    #[test]
    fn ccm_dma_buffer_flagged() {
        let code = r#"static uint8_t __attribute__((section(".ccmram"))) dma_buf[64];
void f(void) { HAL_DMA_Start_IT(dma_buf, 2); }"#;
        let v = violations(code);
        assert_eq!(v.len(), 1, "{v:?}");
        assert_eq!(v[0].rule, "dma_buffer_in_ccmram");
        assert_eq!(v[0].severity, "error");
        assert_eq!(v[0].position.line, 1);
    }

    #[test]
    fn ccm_buffer_without_dma_use_not_flagged() {
        let code = r#"static uint32_t __attribute__((section(".ccmram"))) ema_state;
void f(void) { ema_state = 5; }"#;
        assert!(violations(code).is_empty());
    }

    #[test]
    fn nvic_group_4_not_flagged() {
        let code = "void f(void) { HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_4); }";
        assert!(violations(code).is_empty());
    }

    #[test]
    fn nvic_group_2_flagged_with_position() {
        let code = "void setup(void)\n{\n  HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_2);\n}";
        let v = violations(code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule, "nvic_priority_grouping");
        assert_eq!(v[0].severity, "warning");
        assert_eq!(v[0].position.line, 3);
        let snippet = &code[v[0].position.byte_start..v[0].position.byte_end];
        assert_eq!(snippet, "HAL_NVIC_SetPriorityGrouping");
    }

    #[test]
    fn usb_init_without_pll48_flagged() {
        let code = "void f(void) { HAL_PCD_Init(&hpcd); }";
        let v = violations(code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule, "usb_48mhz_required");
        assert_eq!(v[0].severity, "error");
    }

    #[test]
    fn usb_init_with_pllq_ok() {
        let code = "void f(void) { HAL_PCD_Init(&hpcd); PLLQ = 7; }";
        assert!(violations(code).is_empty());
    }

    #[test]
    fn rop_level2_flagged() {
        let code = "void f(void) { FLASH_OB_RDP_LEVEL_2; }";
        let v = violations(code);
        assert_eq!(v.len(), 1);
        assert_eq!(v[0].rule, "flash_rop_level2");
        let snippet = &code[v[0].position.byte_start..v[0].position.byte_end];
        assert_eq!(snippet, "FLASH_OB_RDP_LEVEL_2");
    }

    #[test]
    fn valid_dma_code_has_no_violations() {
        let code = r#"static uint8_t adc_dma_buf[128];
void start(void) { HAL_ADC_Start_DMA(&hadc1, adc_dma_buf, 128); }"#;
        assert!(violations(code).is_empty());
    }

    #[test]
    fn flow_tracks_dma_usage() {
        let code = r#"static uint8_t __attribute__((section(".ccmram"))) buf[64];
void start(void) { HAL_DMA_Start_IT(buf, 2); }"#;
        let r = run(code, true);
        assert_eq!(r.flows.len(), 1);
        let f = &r.flows[0];
        assert_eq!(f.variable, "buf");
        assert!(f.reaches_dma);
        assert_eq!(f.uses.len(), 2);
        assert_eq!(f.uses[0].kind, "declaration");
        assert_eq!(f.uses[1].kind, "dma_call");
    }

    #[test]
    fn flow_declaration_only_does_not_reach_dma() {
        let code = r#"static uint8_t __attribute__((section(".ccmram"))) buf[64];
void f(void) { buf[0] = 1; }"#;
        let r = run(code, true);
        assert_eq!(r.flows.len(), 1);
        assert!(!r.flows[0].reaches_dma);
    }

    #[test]
    fn summary_counts_severities() {
        let code = "void f(void) { HAL_NVIC_SetPriorityGrouping(NVIC_PRIORITYGROUP_1); FLASH_OB_RDP_LEVEL_2; }";
        let v = violations(code);
        let s = summarize(&v);
        assert_eq!(s.errors, 1);
        assert_eq!(s.warnings, 1);
    }
}
