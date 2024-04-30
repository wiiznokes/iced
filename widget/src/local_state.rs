
use crate::core::event::{self, Event};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::theme::palette;
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::widget::Operation;
use crate::core::{
    Background, Border, Clipboard, Color, Element, Layout, Length, Padding,
    Rectangle, Shadow, Shell, Size, Theme, Vector, Widget,
};


#[allow(missing_debug_implementations)]
pub struct LocalState<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: crate::core::Renderer,
{
    content: Element<'a, Message, Theme, Renderer>,
}

impl<'a, Message, Theme, Renderer> LocalState<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        let content = content.into();

        Button {
            content,
        }
    }

   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
}

impl<'a, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for LocalState<'a, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + crate::core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(std::slice::from_ref(&self.content));
    }

    fn size(&self) -> Size<Length> {
        Size::ZERO
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.as_widget().operate(
                &mut tree.children[0],
                layout.children().next().unwrap(),
                renderer,
                operation,
            );
        });
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.content.as_widget_mut().on_event(
            &mut tree.children[0],
            event.clone(),
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            &renderer::Style {
                text_color: style.text_color,
            },
            content_layout,
            cursor,
            &viewport,
        );
    }


    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, Message, Theme, Renderer> From<LocalState<'a, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: crate::core::Renderer + 'a,
{
    fn from(local_state: LocalState<'a, Message, Theme, Renderer>) -> Self {
        Self::new(local_state)
    }
}