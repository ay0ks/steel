#[derive(Clone, Debug, Default)]
pub struct Module {
    pub name: String,
    pub filename: String,
    pub source: String,
    pub imports: Vec<String>,
    pub externs: Vec<String>,
    pub exports: Vec<String>,
    pub statements: Vec<String>,
    pub requires: Vec<Module>,
}
