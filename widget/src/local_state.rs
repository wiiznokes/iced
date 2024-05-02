
use std::borrow::{BorrowMut, Borrow};
use std::cell::RefCell;
use std::sync::Arc;

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


type Maker<'a, T, Message, Theme, Renderer> = fn(&mut T) -> Element<'a, Message, Theme, Renderer>;

#[allow(missing_debug_implementations)]
pub struct LocalState<'a, T, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: crate::core::Renderer,
{
    state: T,
    maker: Maker<'a, T, Message, Theme, Renderer>,
    content: Arc<Option<Element<'a, Message, Theme, Renderer>>>
}

impl<'a, T, Message, Theme, Renderer> LocalState<'a, T, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    pub fn new(
        default: T,
        content: Maker<'a, T, Message, Theme, Renderer>,
    ) -> Self {

        Self {
            maker: content,
            content: Arc::new(None),
            state: default
        }
    }

   
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State<T> {
    pub inner: T
}

impl<'a, T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for LocalState<'a, T, Message, Theme, Renderer>
where
    Message: 'a + Clone,
    Renderer: 'a + crate::core::Renderer,
    T: 'static + Clone,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<T>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            inner: self.state.clone(),
        })
    }

    // call fun here
    fn children(&self) -> Vec<Tree> {


        let content = (self.maker)(&mut state.inner);
        
        let e = self.content.as_ref().as_ref().unwrap();

        vec![Tree::new(e)]
    }

    fn diff(&self, tree: &mut Tree) {
        let e = self.content.as_ref().as_ref().unwrap();

        tree.diff_children(std::slice::from_ref(e));
    }

    fn size(&self) -> Size<Length> {
        Size {
            // todo: use the size child ?
            width: Length::Fixed(0.),
            height: Length::Fixed(0.),
        }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {


        let state = tree.state.downcast_mut::<State<T>>();
        
        let content = (self.maker)(&mut state.inner);
        
        let node = content.as_widget().layout(
            &mut tree.children[0],
            renderer,
            limits,
        );

        self.content.borrow_mut().replace(content);

        node
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(None, layout.bounds(), &mut |operation| {
            self.content.unwrap().as_widget().operate(
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

        self.content.borrow_mut().as_mut().unwrap().as_widget_mut().on_event(
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
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_ref().unwrap().as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }


    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.borrow_mut().as_mut().unwrap().as_widget_mut().overlay(
            &mut tree.children[0],
            layout.children().next().unwrap(),
            renderer,
            translation,
        )
    }
}

impl<'a, T, Message, Theme, Renderer> From<LocalState<'a, T, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: Clone + 'a,
    Renderer: crate::core::Renderer + 'a,
    T: 'static + Clone,
{
    fn from(local_state: LocalState<'a, T, Message, Theme, Renderer>) -> Self {
        Self::new(local_state)
    }
}