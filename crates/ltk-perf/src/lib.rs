//! # ltk-perf — Profiler · Frame Analyzer · Memory Analyzer · Render Optimizer
pub mod profiler;
pub mod frame_analyzer;
pub mod memory_analyzer;
pub mod render_optimizer;

pub use profiler::{Profiler, Span, SpanGuard};
pub use frame_analyzer::{FrameAnalyzer, FrameReport};
pub use memory_analyzer::MemoryAnalyzer;
