//! Per-frame performance breakdown with budget warnings.

use std::collections::VecDeque;

/// Time breakdown for a single rendered frame.
#[derive(Debug, Clone, Default)]
pub struct FrameReport {
    pub frame_nr:      u64,
    pub input_us:      f64,
    pub event_us:      f64,
    pub state_us:      f64,
    pub animation_us:  f64,
    pub layout_us:     f64,
    pub scene_us:      f64,
    pub paint_us:      f64,
    pub commit_us:     f64,
    pub a11y_us:       f64,
    pub total_us:      f64,
    pub budget_us:     f64,
}

impl FrameReport {
    pub fn is_over_budget(&self) -> bool { self.total_us > self.budget_us }
    pub fn budget_utilization(&self) -> f64 {
        if self.budget_us > 0.0 { self.total_us / self.budget_us } else { 0.0 }
    }
}

pub struct FrameAnalyzer {
    history:    VecDeque<FrameReport>,
    max_history:usize,
}

impl FrameAnalyzer {
    pub fn new(max_history: usize) -> Self { Self { history: VecDeque::new(), max_history } }

    pub fn push(&mut self, report: FrameReport) {
        if self.history.len() >= self.max_history { self.history.pop_front(); }
        if report.is_over_budget() {
            log::warn!("Frame {} over budget: {:.2}ms > {:.2}ms",
                report.frame_nr, report.total_us / 1000.0, report.budget_us / 1000.0);
        }
        self.history.push_back(report);
    }

    pub fn average_fps(&self) -> f64 {
        if self.history.is_empty() { return 0.0; }
        let avg_us: f64 = self.history.iter().map(|r| r.total_us).sum::<f64>() / self.history.len() as f64;
        if avg_us > 0.0 { 1_000_000.0 / avg_us } else { 0.0 }
    }

    pub fn worst_frame(&self) -> Option<&FrameReport> {
        self.history.iter().max_by(|a, b| a.total_us.partial_cmp(&b.total_us).unwrap())
    }

    pub fn history(&self) -> &VecDeque<FrameReport> { &self.history }
}

impl Default for FrameAnalyzer { fn default() -> Self { Self::new(300) } }
