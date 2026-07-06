//! Instrumented spans for flame graph profiling.

use ltk_core::time::Instant;
use std::collections::VecDeque;

/// A completed profiling span.
#[derive(Debug, Clone)]
pub struct Span {
    pub name:     &'static str,
    pub start_ns: u64,
    pub end_ns:   u64,
    pub depth:    u32,
    pub thread:   u64,
}

impl Span {
    pub fn duration_us(&self) -> f64 { (self.end_ns - self.start_ns) as f64 / 1_000.0 }
    pub fn duration_ms(&self) -> f64 { self.duration_us() / 1_000.0 }
}

/// RAII guard: records a span on drop.
pub struct SpanGuard<'p> {
    profiler: &'p Profiler,
    name:     &'static str,
    start_ns: u64,
    depth:    u32,
}

impl<'p> Drop for SpanGuard<'p> {
    fn drop(&mut self) {
        let end_ns = nanos_now();
        self.profiler.record(Span {
            name: self.name,
            start_ns: self.start_ns,
            end_ns,
            depth: self.depth,
            thread: thread_id(),
        });
    }
}

/// Per-thread profiling accumulator.
pub struct Profiler {
    spans:    parking_lot::Mutex<VecDeque<Span>>,
    depth:    std::sync::atomic::AtomicU32,
    max_spans:usize,
    enabled:  bool,
}

impl Profiler {
    pub fn new(max_spans: usize) -> Self {
        Self { spans: Default::default(), depth: Default::default(), max_spans, enabled: cfg!(debug_assertions) }
    }

    pub fn begin(&self, name: &'static str) -> SpanGuard<'_> {
        let depth = self.depth.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        SpanGuard { profiler: self, name, start_ns: nanos_now(), depth }
    }

    pub fn record(&self, span: Span) {
        if !self.enabled { return; }
        let mut q = self.spans.lock();
        if q.len() >= self.max_spans { q.pop_front(); }
        q.push_back(span);
        self.depth.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
    }

    pub fn drain(&self) -> Vec<Span> { self.spans.lock().drain(..).collect() }
    pub fn span_count(&self) -> usize { self.spans.lock().len() }
}

impl Default for Profiler { fn default() -> Self { Self::new(4096) } }

fn nanos_now() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_nanos() as u64
}

fn thread_id() -> u64 {
    let id = std::thread::current().id();
    format!("{:?}", id).chars().filter(|c| c.is_numeric()).collect::<String>().parse().unwrap_or(0)
}
