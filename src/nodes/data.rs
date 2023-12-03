use eframe::egui::{self, DragValue};
use eframe::epaint::ecolor::Color32;
use egui_node_graph::{
    DataTypeTrait, NodeDataTrait, NodeId, NodeResponse, UserResponseTrait, WidgetValueTrait,
};
use naga::TypeInner;
use serde::{Deserialize, Serialize};

use super::template::ShaderNodeTemplate;
use super::ShaderNodeGraphState;

#[derive(Serialize, Deserialize)]
pub struct ShaderNodeData {
    pub(super) template: ShaderNodeTemplate,
}

#[derive(Serialize, Deserialize, PartialEq, Eq)]
pub enum ShaderDataType {
    Scalar,
    Vector,
    Matrix,
    Atomic,
    Pointer,
    ValuePointer,
    Array,
    Struct,
    Image,
    Sampler,
    AccelerationStructure,
    RayQuery,
    BindingArray,
}

// #[derive(Clone, Debug, Default, Serialize, Deserialize)]
// pub enum ShaderValueType {
//     #[default]
//     None,
//     Color(Color32),
//     Vec3(f32, f32, f32),
//     AddColor(Color32, Color32),
// }

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShaderNodeResponse {
    SetActiveNode(NodeId),
    ClearActiveNode,
}
impl UserResponseTrait for ShaderNodeResponse {}

impl DataTypeTrait<ShaderNodeGraphState> for ShaderDataType {
    fn data_type_color(&self, _user_state: &mut ShaderNodeGraphState) -> Color32 {
        match self {
            ShaderDataType::Array
            | ShaderDataType::Scalar
            | ShaderDataType::Vector
            | ShaderDataType::Matrix
            | ShaderDataType::BindingArray => Color32::LIGHT_GRAY,
            ShaderDataType::Image | ShaderDataType::Sampler => Color32::LIGHT_RED,
            _ => Color32::LIGHT_GREEN,
        }
    }

    fn name(&self) -> std::borrow::Cow<str> {
        std::borrow::Cow::Borrowed(match self {
            ShaderDataType::Array
            | ShaderDataType::Scalar
            | ShaderDataType::Vector
            | ShaderDataType::Matrix
            | ShaderDataType::BindingArray => "Math",
            ShaderDataType::Image | ShaderDataType::Sampler => "Image",
            _ => "",
        })
    }
}

impl WidgetValueTrait for TypeInner {
    type Response = ShaderNodeResponse;
    type UserState = ShaderNodeGraphState;
    type NodeData = ShaderNodeData;

    fn value_widget(
        &mut self,
        param_name: &str,
        node_id: NodeId,
        ui: &mut egui::Ui,
        user_state: &mut Self::UserState,
        node_data: &Self::NodeData,
    ) -> Vec<Self::Response> {
        match self {
            ShaderValueType::None => {}
            ShaderValueType::Color(_) => {}
            ShaderValueType::Vec3(x, y, z) => {
                ui.label(param_name);
                ui.horizontal(|ui| {
                    ui.label("x");
                    ui.add(DragValue::new(x));
                    ui.label("y");
                    ui.add(DragValue::new(y));
                    ui.label("z");
                    ui.add(DragValue::new(z));
                });
            }
            ShaderValueType::AddColor(a, b) => {}
        }

        Vec::new()
    }
}

impl NodeDataTrait for ShaderNodeData {
    type Response = ShaderNodeResponse;
    type UserState = ShaderNodeGraphState;
    type DataType = ShaderDataType;
    type ValueType = TypeInner;

    fn bottom_ui(
        &self,
        ui: &mut egui::Ui,
        node_id: egui_node_graph::NodeId,
        _graph: &egui_node_graph::Graph<Self, Self::DataType, Self::ValueType>,
        user_state: &mut Self::UserState,
    ) -> Vec<egui_node_graph::NodeResponse<Self::Response, Self>>
    where
        Self::Response: egui_node_graph::UserResponseTrait,
    {
        let mut responses = vec![];
        let is_active = user_state
            .active_node
            .map(|id| id == node_id)
            .unwrap_or(false);

        if !is_active {
            if ui.button("👁 Set active").clicked() {
                responses.push(NodeResponse::User(ShaderNodeResponse::SetActiveNode(
                    node_id,
                )));
            }
        } else {
            let button =
                egui::Button::new(egui::RichText::new("👁 Active").color(egui::Color32::BLACK))
                    .fill(egui::Color32::GOLD);
            if ui.add(button).clicked() {
                responses.push(NodeResponse::User(ShaderNodeResponse::ClearActiveNode));
            }
        }

        responses
    }
}
