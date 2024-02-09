use dioxus::prelude::*;

fn make_config() -> dioxus_desktop::Config {
    let main_menu = dioxus_desktop::tao::menu::MenuBar::new();
    dioxus_desktop::Config::default().with_window(
        dioxus_desktop::WindowBuilder::new()
            .with_title("Segfaulty")
            .with_min_inner_size(dioxus_desktop::tao::dpi::Size::Logical(
                dioxus_desktop::tao::dpi::LogicalSize {
                    width: 900.0,
                    height: 600.0,
                },
            ))
            .with_menu(main_menu),
            )
}

fn App(cx: Scope) -> Element {
    let desktop = dioxus_desktop::use_window(cx);
    cx.render(rsx!{
        div {}
    })
}

fn main() {
    dioxus_desktop::launch_cfg(App, make_config());
}
