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
        Self { byte_start: 0, byte_end: 0, line: 1, col: 1 }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Response {
    Analyze(AnalyzeResponse),
    Lint(LintResponse),
    FlowAnalysis(LintResponse),
    GenerateDocs(DocsResponse),
    RagSearch(RagResponse),
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
