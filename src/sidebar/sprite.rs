use crate::{
    gui::{BuildContext, Ui, UiMessage, UiNode},
    scene::{SceneCommand, SetSpriteColorCommand, SetSpriteRotationCommand, SetSpriteSizeCommand},
    sidebar::{
        make_color_input_field, make_f32_input_field, make_text_mark, COLUMN_WIDTH, ROW_HEIGHT,
    },
    Message,
};
use rg3d::core::scope_profile;
use rg3d::{
    core::pool::Handle,
    gui::{
        grid::{Column, GridBuilder, Row},
        message::{
            ColorFieldMessage, MessageDirection, NumericUpDownMessage, UiMessageData, WidgetMessage,
        },
        widget::WidgetBuilder,
    },
    scene::node::Node,
};
use std::sync::mpsc::Sender;

pub struct SpriteSection {
    pub section: Handle<UiNode>,
    size: Handle<UiNode>,
    rotation: Handle<UiNode>,
    color: Handle<UiNode>,
    sender: Sender<Message>,
}

impl SpriteSection {
    pub fn new(ctx: &mut BuildContext, sender: Sender<Message>) -> Self {
        let size;
        let rotation;
        let color;
        let section = GridBuilder::new(
            WidgetBuilder::new()
                .with_child(make_text_mark(ctx, "Size", 0))
                .with_child({
                    size = make_f32_input_field(ctx, 0, 0.0, std::f32::MAX, 0.1);
                    size
                })
                .with_child(make_text_mark(ctx, "Rotation", 1))
                .with_child({
                    rotation = make_f32_input_field(ctx, 1, 0.0, std::f32::MAX, 0.1);
                    rotation
                })
                .with_child(make_text_mark(ctx, "Color", 2))
                .with_child({
                    color = make_color_input_field(ctx, 2);
                    color
                }),
        )
        .add_column(Column::strict(COLUMN_WIDTH))
        .add_column(Column::stretch())
        .add_row(Row::strict(ROW_HEIGHT))
        .add_row(Row::strict(ROW_HEIGHT))
        .add_row(Row::strict(ROW_HEIGHT))
        .build(ctx);

        Self {
            section,
            size,
            rotation,
            sender,
            color,
        }
    }

    pub fn sync_to_model(&mut self, node: &Node, ui: &mut Ui) {
        ui.send_message(WidgetMessage::visibility(
            self.section,
            MessageDirection::ToWidget,
            node.is_sprite(),
        ));

        if let Node::Sprite(sprite) = node {
            ui.send_message(NumericUpDownMessage::value(
                self.size,
                MessageDirection::ToWidget,
                sprite.size(),
            ));

            ui.send_message(NumericUpDownMessage::value(
                self.rotation,
                MessageDirection::ToWidget,
                sprite.rotation(),
            ));

            ui.send_message(ColorFieldMessage::color(
                self.color,
                MessageDirection::ToWidget,
                sprite.color(),
            ));
        }
    }

    pub fn handle_message(&mut self, message: &UiMessage, node: &Node, handle: Handle<Node>) {
        scope_profile!();

        if let Node::Sprite(sprite) = node {
            match &message.data() {
                UiMessageData::NumericUpDown(msg) => {
                    if let NumericUpDownMessage::Value(value) = *msg {
                        if message.destination() == self.size && sprite.size().ne(&value) {
                            self.sender
                                .send(Message::DoSceneCommand(SceneCommand::SetSpriteSize(
                                    SetSpriteSizeCommand::new(handle, value),
                                )))
                                .unwrap();
                        } else if message.destination() == self.rotation
                            && sprite.rotation().ne(&value)
                        {
                            self.sender
                                .send(Message::DoSceneCommand(SceneCommand::SetSpriteRotation(
                                    SetSpriteRotationCommand::new(handle, value),
                                )))
                                .unwrap();
                        }
                    }
                }
                UiMessageData::ColorField(msg) => {
                    if let ColorFieldMessage::Color(color) = *msg {
                        if message.destination() == self.color && sprite.color() != color {
                            self.sender
                                .send(Message::DoSceneCommand(SceneCommand::SetSpriteColor(
                                    SetSpriteColorCommand::new(handle, color),
                                )))
                                .unwrap();
                        }
                    }
                }
                _ => {}
            }
        }
    }
}
