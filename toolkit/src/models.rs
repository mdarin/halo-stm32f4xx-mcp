use serde::{Deserialize, Serialize};

pub const SCHEMA_VERSION: u32 = 1;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct Position {
    pub byte_start: usize,
    pub byte_end: usize,
    pub line: usize,
    pub col: usize,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            byte_start: 0,
            byte_end: 0,
            line: 1,
            col: 1,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub struct SourceStats {
    pub bytes: usize,
    pub lines: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attribute {
    pub name: String,
    #[serde(default)]
    pub args: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    pub name: String,
    pub signature: String,
    pub position: Position,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableInfo {
    pub name: String,
    pub r#type: String,
    pub storage: String,
    pub position: Position,
    #[serde(default)]
    pub attributes: Vec<Attribute>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HalCall {
    pub callee: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParseError {
    pub message: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalyzeResponse {
    pub schema_version: u32,
    pub command: String,
    pub source_stats: SourceStats,
    #[serde(default)]
    pub functions: Vec<FunctionInfo>,
    #[serde(default)]
    pub variables: Vec<VariableInfo>,
    #[serde(default)]
    pub hal_calls: Vec<HalCall>,
    #[serde(default)]
    pub parse_errors: Vec<ParseError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Violation {
    pub rule: String,
    pub severity: String,
    pub message: String,
    pub position: Position,
    pub hint: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FlowUse {
    pub kind: String,
    pub position: Position,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flow {
    pub variable: String,
    pub uses: Vec<FlowUse>,
    pub reaches_dma: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Default)]
pub struct Summary {
    pub errors: usize,
    pub warnings: usize,
    pub infos: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LintResponse {
    pub schema_version: u32,
    pub command: String,
    pub violations: Vec<Violation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<Vec<Flow>>,
    pub summary: Summary,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Insertion {
    pub position: Position,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocsResponse {
    pub schema_version: u32,
    pub command: String,
    pub insertions: Vec<Insertion>,
    pub functions_documented: usize,
    pub functions_skipped: usize,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RagRef {
    pub score: f64,
    pub source: String,
    pub title: String,
    pub chapter: u32,
    pub section: String,
    pub snippet: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagResponse {
    pub schema_version: u32,
    pub command: String,
    pub violations: Vec<Violation>,
    pub rag_refs: Vec<RagRef>,
    pub rag_degraded: bool,
}

/// Хит поиска по документации (CONTRACT.md §6). Поля зеркалят payload точек
/// Qdrant: source берётся из source_file, chapter — число.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocHit {
    pub text: String,
    pub source: String,
    pub peripheral: String,
    pub chapter: i64,
    pub section: String,
    pub title: String,
    pub score: f64,
}

/// Ответ rag-query (CONTRACT.md §6) — поиск по документации без AST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagQueryResponse {
    pub schema_version: u32,
    pub command: String,
    pub query: String,
    pub peripheral: String,
    pub results: Vec<DocHit>,
}

/// Ответ rag-upsert (CONTRACT.md §7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagUpsertResponse {
    pub schema_version: u32,
    pub command: String,
    pub upserted: usize,
}

/// Ответ rag-index (CONTRACT.md §7).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RagIndexResponse {
    pub schema_version: u32,
    pub command: String,
    pub files: usize,
    pub chunks: usize,
    pub skipped: usize,
    pub errors: usize,
}

/// Хит ctx-search (CONTRACT.md §10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxHit {
    pub score: f64,
    pub project_path: String,
    pub ctx_key: String,
    pub doc_type: String,
    pub text: String,
}

/// Ответ ctx-save (CONTRACT.md §10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxSaveResponse {
    pub schema_version: u32,
    pub command: String,
    pub saved: usize,
    pub collection: String,
}

/// Запись контекста в ответе ctx-load.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxEntry {
    pub ctx_key: String,
    pub doc_type: String,
    pub text: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub peripherals: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub interrupts: Vec<String>,
    #[serde(default, skip_serializing_if = "std::collections::HashMap::is_empty")]
    pub user_preferences: std::collections::HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub modification: Option<crate::context::Modification>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<i64>,
}

/// Ответ ctx-load (CONTRACT.md §10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxLoadResponse {
    pub schema_version: u32,
    pub command: String,
    pub project_path: String,
    pub count: usize,
    pub entries: Vec<CtxEntry>,
}

/// Ответ ctx-search (CONTRACT.md §10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxSearchResponse {
    pub schema_version: u32,
    pub command: String,
    pub project_path: String,
    pub query: String,
    pub results: Vec<CtxHit>,
}

/// USER CODE блок, найденный в исходниках (CONTRACT.md §10).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxBlock {
    pub file: String,
    pub name: String,
}

/// Ответ ctx-extract (CONTRACT.md §10): автосбор периферии/IRQ/blocks
/// из .ioc + AST-скана. Diff-поля заполняются при доступном Qdrant.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CtxExtractResponse {
    pub schema_version: u32,
    pub command: String,
    pub project_path: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mcu: Option<String>,
    pub peripherals: Vec<String>,
    pub interrupts: Vec<String>,
    pub files: Vec<String>,
    pub blocks: Vec<CtxBlock>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_peripherals: Option<Vec<String>>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub new_interrupts: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Analyze(AnalyzeResponse),
    Lint(LintResponse),
    FlowAnalysis(LintResponse),
    GenerateDocs(DocsResponse),
    RagSearch(RagResponse),
    RagQuery(RagQueryResponse),
    RagUpsert(RagUpsertResponse),
    RagIndex(RagIndexResponse),
    CtxSave(CtxSaveResponse),
    CtxLoad(CtxLoadResponse),
    CtxSearch(CtxSearchResponse),
    CtxExtract(CtxExtractResponse),
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorCode {
    InputRead,
    Parse,
    SchemaUnsupported,
    RagUnavailable,
    Internal,
}

impl ErrorCode {
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::InputRead => "E_INPUT_READ",
            ErrorCode::Parse => "E_PARSE",
            ErrorCode::SchemaUnsupported => "E_SCHEMA_UNSUPPORTED",
            ErrorCode::RagUnavailable => "E_RAG_UNAVAILABLE",
            ErrorCode::Internal => "E_INTERNAL",
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ToolkitError {
    pub schema_version: u32,
    pub error: ErrorBody,
}

#[derive(Debug, Serialize)]
pub struct ErrorBody {
    pub code: &'static str,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub position: Option<Position>,
    pub command: String,
}

impl ToolkitError {
    pub fn new(code: ErrorCode, message: impl Into<String>, command: &str) -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            error: ErrorBody {
                code: code.as_str(),
                message: message.into(),
                position: None,
                command: command.to_string(),
            },
        }
    }

    #[allow(dead_code)]
    pub fn with_position(mut self, position: Position) -> Self {
        self.error.position = Some(position);
        self
    }

    pub fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| {
            r#"{"schema_version":1,"error":{"code":"E_INTERNAL","message":"error serialization failed","command":""}}"#.into()
        })
    }
}
