use iced::{
    advanced::{
        layout, mouse, renderer::{self, Quad}, widget::Tree, Layout, Widget
    }, Border, Color, Element, Length, Rectangle, Shadow, Size, Theme
};

pub struct WhiteFrame<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
    on_press: Option<Message>,
}

impl <'a, Message, Renderer> WhiteFrame<'a, Message, Renderer>
where 
    Renderer: iced::advanced::Renderer,
{
    pub fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self { 
            content,
            on_press: None,
        }
    }
    
    pub fn on_press(mut self, message: Message) -> Self {
        self.on_press = Some(message);

        self
    }
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for WhiteFrame<'a, Message, Renderer> 
where 
    Renderer: iced::advanced::Renderer,
    Message: Clone,
{
    fn size(&self) -> Size<Length> {
        Size {
            width: Length::Fill,
            height: Length::Fill,
        }
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        let mut child_node = 
            self.content
                .as_widget()
                .layout(&mut tree.children[0], renderer, limits);
        
        let size_of_this_node = limits.max();

        child_node = child_node.align(iced::Alignment::Start, iced::Alignment::Start, size_of_this_node);

        layout::Node::with_children(size_of_this_node, vec![child_node])
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(self.content.as_widget())]
    }

    fn operate(
            &self,
            state: &mut Tree,
            layout: Layout<'_>,
            renderer: &Renderer,
            operation: &mut dyn iced::advanced::widget::Operation,
        ) {
            operation.container(None, layout.bounds(), &mut |operation| {
                    self.content
                        .as_widget()
                        .operate(&mut state.children[0], layout.children().next().unwrap(), renderer, operation);
                });
    }

    fn on_event(
            &mut self,
            state: &mut Tree,
            event: iced::Event,
            layout: Layout<'_>,
            cursor: mouse::Cursor,
            renderer: &Renderer,
            clipboard: &mut dyn iced::advanced::Clipboard,
            shell: &mut iced::advanced::Shell<'_, Message>,
            viewport: &Rectangle,
        ) -> iced::advanced::graphics::core::event::Status {
            if let iced::Event::Mouse(mouse_event) = &event {
                if let mouse::Event::ButtonPressed(mouse::Button::Left) = mouse_event {
                    if let Some(cursor_position) = cursor.position() {
                        if layout.bounds().contains(cursor_position) {
                            if let Some(message) = self.on_press.clone() {
                                shell.publish(message);
                                return iced::advanced::graphics::core::event::Status::Captured;
                            }
                        }
                    }
                }
            }
            self.content.as_widget_mut().on_event(
                &mut state.children[0],
                event,
                layout.children().next().unwrap(),
                cursor,
                renderer,
                clipboard,
                shell,
                viewport
            )
        }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        renderer.fill_quad(
            Quad {
                bounds: layout.bounds(),
                border: Border {
                    color: Color::from_rgb(1.0, 1.0, 1.0),
                    width: 0.0,
                    radius: 5.0.into(),
                },
                shadow: Shadow::default(),
            },
            Color::from_rgb(1., 1., 1.),
        );

        self.content.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            viewport,
        );
    }
}

impl<'a, Message, Renderer> From<WhiteFrame<'a, Message, Renderer>> for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(widget: WhiteFrame<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}