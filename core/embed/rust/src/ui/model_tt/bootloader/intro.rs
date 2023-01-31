use crate::ui::{
    component::{
        text::paragraphs::{Paragraph, ParagraphVecShort, Paragraphs, VecExt},
        Child, Component, Event, EventCtx, Label, Pad,
    },
    display::Icon,
    geometry::{Alignment, Insets, LinearPlacement, Point, Rect},
    model_tt::{
        bootloader::theme::{button_bld_menu, button_bld_menu_item, BLD_BG, MENU, TEXT_NORMAL},
        component::ButtonMsg::Clicked,
    },
};
use heapless::String;

use crate::ui::model_tt::{
    bootloader::theme::TEXT_TITLE,
    component::Button,
    constant::{HEIGHT, WIDTH},
};

#[repr(u32)]
#[derive(Copy, Clone, ToPrimitive)]
pub enum IntroMsg {
    Menu = 1,
    Host = 2,
}

pub struct Intro {
    bg: Pad,
    title: Child<Label<String<20>>>,
    menu: Child<Button<&'static str>>,
    host: Child<Button<&'static str>>,
    text: Child<Paragraphs<ParagraphVecShort<&'static str>>>,
}

impl Intro {
    pub fn new(bld_version: &'static str, vendor: &'static str, version: &'static str) -> Self {
        let mut messages = ParagraphVecShort::new();

        messages.add(Paragraph::new(&TEXT_NORMAL, version));
        messages.add(Paragraph::new(&TEXT_NORMAL, vendor));

        let p =
            Paragraphs::new(messages).with_placement(LinearPlacement::vertical().align_at_start());

        let mut title: String<20> = String::new();
        unwrap!(title.push_str("BOOTLOADER "));
        unwrap!(title.push_str(bld_version));

        let mut instance = Self {
            bg: Pad::with_background(BLD_BG),
            title: Child::new(Label::new(title, Alignment::Start, TEXT_TITLE)),
            menu: Child::new(
                Button::with_icon(Icon::new(MENU))
                    .styled(button_bld_menu())
                    .with_expanded_touch_area(Insets::uniform(13)),
            ),
            host: Child::new(Button::with_text("INSTALL FIRMWARE").styled(button_bld_menu_item())),
            text: Child::new(p),
        };

        instance.bg.clear();
        instance
    }
}

impl Component for Intro {
    type Msg = IntroMsg;

    fn place(&mut self, bounds: Rect) -> Rect {
        self.bg
            .place(Rect::new(Point::new(0, 0), Point::new(WIDTH, HEIGHT)));
        self.title
            .place(Rect::new(Point::new(15, 24), Point::new(180, 40)));
        self.menu.place(Rect::new(
            Point::new(187, 15),
            Point::new(187 + 38, 15 + 38),
        ));
        self.host.place(Rect::new(
            Point::new(16, 178),
            Point::new(16 + 209, 178 + 48),
        ));
        self.text
            .place(Rect::new(Point::new(15, 75), Point::new(225, 200)));
        bounds
    }

    fn event(&mut self, ctx: &mut EventCtx, event: Event) -> Option<Self::Msg> {
        if let Some(Clicked) = self.menu.event(ctx, event) {
            return Some(Self::Msg::Menu);
        };
        if let Some(Clicked) = self.host.event(ctx, event) {
            return Some(Self::Msg::Host);
        };
        None
    }

    fn paint(&mut self) {
        self.bg.paint();
        self.title.paint();
        self.text.paint();
        self.host.paint();
        self.menu.paint();
    }

    fn bounds(&self, sink: &mut dyn FnMut(Rect)) {
        self.menu.bounds(sink);
    }
}
