//! Scene graph: visual properties, transforms, clips, layer ordering.

use ltk_core::{
    geometry::{Rect, Transform2D},
    color::Color,
    id::WidgetId,
};
use smallvec::SmallVec;

pub type SceneNodeId = WidgetId;

/// Visual properties stored on each scene node.
#[derive(Debug, Clone)]
pub struct SceneNode {
    pub id:          SceneNodeId,
    pub rect:        Rect,
    pub transform:   Transform2D,
    pub opacity:     f32,           // 0.0–1.0
    pub clip_rect:   Option<Rect>,
    pub clip_radius: f32,
    pub z_index:     i32,
    pub visible:     bool,
    pub blend_mode:  BlendMode,
    pub children:    SmallVec<[SceneNodeId; 8]>,
    pub parent:      Option<SceneNodeId>,
    // Cached world transform (recomputed when dirty)
    pub world_transform: Transform2D,
    pub dirty:       bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum BlendMode { #[default] Normal, Multiply, Screen, Overlay, Darken, Lighten }

impl SceneNode {
    pub fn new(id: SceneNodeId) -> Self {
        Self {
            id, rect: Rect::ZERO, transform: Transform2D::IDENTITY,
            opacity: 1.0, clip_rect: None, clip_radius: 0.0,
            z_index: 0, visible: true, blend_mode: BlendMode::Normal,
            children: SmallVec::new(), parent: None,
            world_transform: Transform2D::IDENTITY, dirty: true,
        }
    }
}

/// The complete scene graph for one window.
pub struct SceneGraph {
    nodes: std::collections::HashMap<SceneNodeId, SceneNode>,
    root:  Option<SceneNodeId>,
}

impl SceneGraph {
    pub fn new() -> Self { Self { nodes: Default::default(), root: None } }

    pub fn insert(&mut self, node: SceneNode) { self.nodes.insert(node.id, node); }
    pub fn get(&self, id: SceneNodeId) -> Option<&SceneNode> { self.nodes.get(&id) }
    pub fn get_mut(&mut self, id: SceneNodeId) -> Option<&mut SceneNode> { self.nodes.get_mut(&id) }
    pub fn set_root(&mut self, id: SceneNodeId) { self.root = Some(id); }
    pub fn root(&self) -> Option<SceneNodeId> { self.root }

    /// Recompute world transforms for dirty nodes.
    pub fn update_transforms(&mut self) {
        if let Some(root) = self.root {
            self.propagate_transform(root, Transform2D::IDENTITY);
        }
    }

    fn propagate_transform(&mut self, id: SceneNodeId, parent_world: Transform2D) {
        let (local, children) = match self.nodes.get_mut(&id) {
            Some(n) => {
                let world = parent_world.then(n.transform);
                n.world_transform = world;
                n.dirty = false;
                (world, n.children.to_vec())
            }
            None => return,
        };
        for child in children { self.propagate_transform(child, local); }
    }
}

impl Default for SceneGraph { fn default() -> Self { Self::new() } }
