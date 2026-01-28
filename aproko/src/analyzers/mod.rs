//! Analysis engines for different categories of code analysis

pub mod syntax;
pub mod logic;
pub mod performance;
pub mod security;
pub mod correctness;
pub mod style;

pub use syntax::SyntaxAnalyzer;
pub use logic::LogicAnalyzer;
pub use performance::PerformanceAnalyzer;
pub use security::SecurityAnalyzer;
pub use correctness::CorrectnessAnalyzer;
pub use style::StyleAnalyzer;