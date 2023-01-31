use crate::ui::{
    component::{
        text::paragraphs::{ParagraphVecShort, Paragraphs},
        Child, Component, Event, EventCtx, Pad,
    },
    constant,
    constant::screen,
    display::{Color, Icon},
    geometry::{Offset, Point, Rect, CENTER},
    model_tt::{
        component::{Button, ButtonMsg::Clicked},
        constant::{HEIGHT, WIDTH},
        theme::WHITE,
    },
};

#[derive(Copy, Clone, ToPrimitive)]
pub enum ConfirmMsg {
    Cancel = 1,
    Confirm = 2,
}

pub struct Confirm<'a> {
    bg: Pad,
    bg_color: Color,
    icon: Option<Icon>,
    message: Child<Paragraphs<ParagraphVecShort<&'a str>>>,
    left: Child<Button<&'static str>>,
    right: Child<Button<&'static str>>,
    confirm_left: bool,
}

impl<'a> Confirm<'a> {
    pub fn new(
        bg_color: Color,
        icon: Option<Icon>,
        message: Paragraphs<ParagraphVecShort<&'a str>>,
        left: Button<&'static str>,
        right: Button<&'static str>,
        confirm_left: bool,
    ) -> Self {
        let mut instance = Self {
            bg: Pad::with_background(bg_color),
            bg_color,
            icon,
            message: Child::new(message),
            left: Child::new(left),
            right: Child::new(right),
            confirm_left,
        };
        instance.bg.clear();
        instance
    }
}

impl<'a> Component for Confirm<'a> {
    type Msg = ConfirmMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg.place(constant::screen());
        self.message.place(Rect::new(
            Point::new(15, 59),
            Point::new(WIDTH - 15, HEIGHT - 64),
        ));

        let button_size = Offset::new(102, 48);
        self.left.place(Rect::from_top_left_and_size(
            Point::new(15, 176),
            button_size,
        ));
        self.right.place(Rect::from_top_left_and_size(
            Point::new(123, 176),
            button_size,
        ));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.left.event(ctx, event) {
            return if self.confirm_left {
                Some(Self::Msg::Confirm)
            } else {
                Some(Self::Msg::Cancel)
            };
        };
        if let Some(Clicked) = self.right.event(ctx, event) {
            return if self.confirm_left {
                Some(Self::Msg::Cancel)
            } else {
                Some(Self::Msg::Confirm)
            };
        };
        None
    }

    fn paint(&mut self) {
        self.bg.paint();

        if let Some(icon) = self.icon {
            icon.draw(
                Point::new(screen().center().x, 45),
                CENTER,
                WHITE,
                self.bg_color,
            );
        }

        self.message.paint();
        self.left.paint();
        self.right.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.left.bounds(sink);
        self.right.bounds(sink);
    }
}
