use egui_node_graph::{Graph, GraphEditorState, NodeTemplateIter, NodeId};
use serde::{Deserialize, Serialize};

use self::data::{ShaderDataType, ShaderNodeData, ShaderValueType};
use self::template::ShaderNodeTemplate;

mod data;
mod template;

pub type ShaderEditorGraph = Graph<ShaderNodeData, ShaderDataType, ShaderValueType>;
pub type ShaderEditorState = GraphEditorState<
    ShaderNodeData,
    ShaderDataType,
    ShaderValueType,
    ShaderNodeTemplate,
    ShaderNodeGraphState,
>;

#[derive(Default, Serialize, Deserialize)]
pub struct ShaderNodeGraphState {
    pub active_node: Option<NodeId>,
}

pub struct ShaderNodeTemplates;
impl NodeTemplateIter for ShaderNodeTemplates {
    type Item = ShaderNodeTemplate;

    fn all_kinds(&self) -> Vec<Self::Item> {
        vec![]
    }
}
