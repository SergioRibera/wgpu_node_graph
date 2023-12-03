use std::borrow::Cow;

use eframe::epaint::Color32;
use egui_node_graph::{InputParamKind, NodeTemplateTrait};
use serde::{Deserialize, Serialize};

use super::data::{ShaderDataType, ShaderNodeData, ShaderValueType};
use super::{ShaderEditorGraph, ShaderNodeGraphState};

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum ShaderNodeTemplate {
    MakeVec3,
    MakeColor,
    MakeAdd,
}

impl NodeTemplateTrait for ShaderNodeTemplate {
    type NodeData = ShaderNodeData;
    type DataType = ShaderDataType;
    type ValueType = ShaderValueType;
    type UserState = ShaderNodeGraphState;
    type CategoryType = &'static str;

    fn node_finder_label(&self, _user_state: &mut Self::UserState) -> std::borrow::Cow<str> {
        Cow::Borrowed(match self {
            ShaderNodeTemplate::MakeVec3 => "New Vec3",
            ShaderNodeTemplate::MakeColor => "New Color",
            ShaderNodeTemplate::MakeAdd => "Add",
        })
    }

    fn node_finder_categories(&self, _user_state: &mut Self::UserState) -> Vec<&'static str> {
        match self {
            ShaderNodeTemplate::MakeVec3 => vec!["Math"],
            ShaderNodeTemplate::MakeAdd | ShaderNodeTemplate::MakeColor => vec!["Color"],
        }
    }

    fn node_graph_label(&self, user_state: &mut Self::UserState) -> String {
        self.node_finder_label(user_state).into()
    }

    fn user_data(&self, _user_state: &mut Self::UserState) -> Self::NodeData {
        ShaderNodeData { template: *self }
    }

    fn build_node(
        &self,
        graph: &mut egui_node_graph::Graph<Self::NodeData, Self::DataType, Self::ValueType>,
        _user_state: &mut Self::UserState,
        node_id: egui_node_graph::NodeId,
    ) {
        let input_vec2 = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ShaderDataType::Vec3,
                ShaderValueType::Vec3(0., 0., 0.),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_color = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ShaderDataType::Color,
                ShaderValueType::Color(Color32::WHITE),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };
        let input_add = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_input_param(
                node_id,
                name.to_string(),
                ShaderDataType::AddColor,
                ShaderValueType::AddColor(Color32::WHITE, Color32::WHITE),
                InputParamKind::ConnectionOrConstant,
                true,
            );
        };

        let output_vec3 = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ShaderDataType::Vec3);
        };
        let output_color = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ShaderDataType::Color);
        };
        let output_add = |graph: &mut ShaderEditorGraph, name: &str| {
            graph.add_output_param(node_id, name.to_string(), ShaderDataType::Color);
        };

        match self {
            ShaderNodeTemplate::MakeVec3 => {
                input_vec2(graph, "Vec3");
                output_vec3(graph, "Out");
            }
            ShaderNodeTemplate::MakeColor => {
                input_color(graph, "Color");
                output_color(graph, "Out");
            }
            ShaderNodeTemplate::MakeAdd => {
                input_add(graph, "A");
                input_add(graph, "B");
                output_add(graph, "Out");
            }
        }
    }
}
