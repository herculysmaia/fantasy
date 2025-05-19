use iced::{
    advanced::{
        layout, mouse, renderer::{self, Quad}, widget::Tree, Layout, Widget
    }, Border, Color, Element, Length, Rectangle, Shadow, Size, Theme
};

pub struct WhiteFrame<'a, Message, Renderer> {
    content: Element<'a, Message, Theme, Renderer>,
}

impl <'a, Message, Renderer> WhiteFrame<'a, Message, Renderer>
where 
    Renderer: iced::advanced::Renderer,
{
    pub fn new(content: Element<'a, Message, Theme, Renderer>) -> Self {
        Self { content }
    }
    
}

impl<'a, Message, Renderer> Widget<Message, Theme, Renderer> for WhiteFrame<'a, Message, Renderer> 
where 
    Renderer: iced::advanced::Renderer,
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
            Color::from_rgb(0.5, 0.5, 0.5),
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
    Message: 'a,
    Renderer: iced::advanced::Renderer + 'a,
{
    fn from(widget: WhiteFrame<'a, Message, Renderer>) -> Self {
        Self::new(widget)
    }
}