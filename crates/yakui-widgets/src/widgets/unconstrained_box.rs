use std::f32::INFINITY;

use yakui_core::dom::Dom;
use yakui_core::geometry::{Constraints, Vec2};
use yakui_core::layout::LayoutDom;
use yakui_core::widget::Widget;
use yakui_core::Response;

use crate::util::widget_children;

/**
A box that renders its child with one or both of its constraint axes ignored.

Responds with [UnconstrainedBoxResponse].
*/
#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct UnconstrainedBox {
    pub constrain_x: bool,
    pub constrain_y: bool,
}

impl UnconstrainedBox {
    pub fn new() -> Self {
        Self {
            constrain_x: false,
            constrain_y: false,
        }
    }

    pub fn show<F: FnOnce()>(self, children: F) -> Response<UnconstrainedBoxWidget> {
        widget_children::<UnconstrainedBoxWidget, F>(children, self)
    }
}

#[derive(Debug)]
pub struct UnconstrainedBoxWidget {
    props: UnconstrainedBox,
}

pub type UnconstrainedBoxResponse = ();

impl Widget for UnconstrainedBoxWidget {
    type Props = UnconstrainedBox;
    type Response = UnconstrainedBoxResponse;

    fn new() -> Self {
        Self {
            props: UnconstrainedBox::new(),
        }
    }

    fn update(&mut self, props: Self::Props) -> Self::Response {
        self.props = props;
    }

    fn layout(&self, dom: &Dom, layout: &mut LayoutDom, input: Constraints) -> Vec2 {
        let node = dom.get_current();

        let (min_x, max_x) = if self.props.constrain_x {
            (0.0, input.max.x)
        } else {
            (0.0, INFINITY)
        };

        let (min_y, max_y) = if self.props.constrain_y {
            (0.0, input.max.y)
        } else {
            (0.0, INFINITY)
        };

        let constraints = Constraints {
            min: Vec2::new(min_x, min_y),
            max: Vec2::new(max_x, max_y),
        };

        let mut size = Vec2::ZERO;
        for &child in &node.children {
            let child_size = layout.calculate(dom, child, constraints);
            size = size.max(child_size);
        }

        input.constrain_min(size)
    }
}
