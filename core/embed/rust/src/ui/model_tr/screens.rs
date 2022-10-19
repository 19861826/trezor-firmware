use crate::ui::{
    component::{text::paragraphs::Paragraphs, Component},
    geometry::LinearPlacement,
    model_tr::{
        component::ResultScreen,
        constant,
        theme::{BLACK, TEXT_BOLD, TEXT_NORMAL, WHITE},
    },
    util::from_c_str,
};

#[no_mangle]
extern "C" fn screen_fatal_error(msg: *const cty::c_char, file: *const cty::c_char) -> u32 {
    let m_top = if msg.is_null() {
        Paragraphs::new()
            .add(TEXT_BOLD, "FATAL ERROR!")
            .centered()
            // .add(theme::TEXT_WIPE_NORMAL, unwrap!(unsafe { from_c_str(expr) }))
            //     .centered()
            .add(TEXT_NORMAL, unwrap!(unsafe { from_c_str(file) }))
            .centered()
            .with_placement(LinearPlacement::vertical().align_at_center())
    } else {
        let msg = unwrap!(unsafe { from_c_str(msg) });
        Paragraphs::new()
            .add(TEXT_BOLD, "FATAL ERROR!")
            .centered()
            .add(TEXT_NORMAL, msg)
            .centered()
            .with_placement(LinearPlacement::vertical().align_at_center())
    };

    let m_bottom = Paragraphs::new()
        .add(TEXT_BOLD, "PLEASE CONTACT")
        .centered()
        .add(TEXT_BOLD, "TREZOR SUPPORT")
        .centered()
        .with_placement(LinearPlacement::vertical().align_at_center());

    let mut frame = ResultScreen::new(WHITE, BLACK, m_top, m_bottom, true);
    frame.place(constant::screen());
    frame.paint();
    0
}

#[no_mangle]
extern "C" fn screen_error_shutdown(label: *const cty::c_char, msg: *const cty::c_char) -> u32 {
    let label = unwrap!(unsafe { from_c_str(label) });

    let m_top = if msg.is_null() {
        Paragraphs::new()
            .add(TEXT_BOLD, label)
            .centered()
            .with_placement(LinearPlacement::vertical().align_at_center())
    } else {
        let msg = unwrap!(unsafe { from_c_str(msg) });
        Paragraphs::new()
            .add(TEXT_BOLD, label)
            .centered()
            .add(TEXT_NORMAL, msg)
            .centered()
            .with_placement(LinearPlacement::vertical().align_at_center())
    };

    let m_bottom = Paragraphs::new()
        .add(TEXT_BOLD, "PLEASE UNPLUG")
        .centered()
        .add(TEXT_BOLD, "THE DEVICE")
        .centered()
        .with_placement(LinearPlacement::vertical().align_at_center());

    let mut frame = ResultScreen::new(WHITE, BLACK, m_top, m_bottom, true);
    frame.place(constant::screen());
    frame.paint();
    0
}
