//! Graphics pipeline state objects (PSO) for common draw types.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PipelineKind { SolidRect, RoundedRect, Text, Image, Blur }

pub struct GraphicsPipeline { pub kind: PipelineKind }

pub struct PipelineCache { pipelines: Vec<GraphicsPipeline> }

impl PipelineCache {
    pub fn new() -> Self { Self { pipelines: Vec::new() } }

    pub fn get_or_create(&mut self, kind: PipelineKind) -> &GraphicsPipeline {
        if let Some(idx) = self.pipelines.iter().position(|p| p.kind == kind) {
            &self.pipelines[idx]
        } else {
            self.pipelines.push(GraphicsPipeline { kind });
            self.pipelines.last().unwrap()
        }
    }
}

impl Default for PipelineCache { fn default() -> Self { Self::new() } }
