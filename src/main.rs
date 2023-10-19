use dioxus::prelude::*;
use dioxus_desktop::tao;
use dioxus_desktop::Config;
use dioxus_desktop::LogicalSize;
use tao::menu::AboutMetadata;
use tao::menu::{MenuBar as Menu, MenuItem};
use tao::window::WindowBuilder;

fn main() {
    dioxus_desktop::launch_cfg(app, webview_config())
}

fn app(cx: Scope) -> Element {
    render! {
       div {
        "hello world"
       }
    }
}

pub(crate) fn webview_config() -> Config {
    let window = get_window_builder(true, true);

    Config::new()
        .with_window(window)
        .with_custom_index(
            r#"
<!doctype html>
<html>
<script src="https://cdn.jsdelivr.net/npm/interactjs/dist/interact.min.js"></script>
<body style="background-color:rgba(0,0,0,0);"><div id="main"></div></body>
</html>"#
                .to_string(),
        )
        .with_file_drop_handler(|_w, drag_event| true)
        .with_disable_context_menu(false)
}

pub fn get_window_builder(with_predefined_size: bool, with_menu: bool) -> WindowBuilder {
    let mut main_menu = Menu::new();
    let mut app_menu = Menu::new();
    let mut edit_menu = Menu::new();
    let mut window_menu = Menu::new();

    app_menu.add_native_item(MenuItem::About(
        String::from("Uplink"),
        AboutMetadata::default(),
    ));
    app_menu.add_native_item(MenuItem::Quit);
    // add native shortcuts to `edit_menu` menu
    // in macOS native item are required to get keyboard shortcut
    // to works correctly
    edit_menu.add_native_item(MenuItem::Undo);
    edit_menu.add_native_item(MenuItem::Redo);
    edit_menu.add_native_item(MenuItem::Separator);
    edit_menu.add_native_item(MenuItem::Cut);
    edit_menu.add_native_item(MenuItem::Copy);
    edit_menu.add_native_item(MenuItem::Paste);
    edit_menu.add_native_item(MenuItem::SelectAll);

    window_menu.add_native_item(MenuItem::Minimize);
    window_menu.add_native_item(MenuItem::Zoom);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::ShowAll);
    window_menu.add_native_item(MenuItem::EnterFullScreen);
    window_menu.add_native_item(MenuItem::Separator);
    window_menu.add_native_item(MenuItem::CloseWindow);

    main_menu.add_submenu("Uplink", true, app_menu);
    main_menu.add_submenu("Edit", true, edit_menu);
    main_menu.add_submenu("Window", true, window_menu);

    let title = "suplink";

    #[allow(unused_mut)]
    let mut window = WindowBuilder::new()
        .with_title(title)
        .with_resizable(true)
        // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
        .with_min_inner_size(LogicalSize::new(300.0, 350.0));

    if with_predefined_size {
        window = window.with_inner_size(LogicalSize::new(950.0, 600.0));
    }

    if with_menu {
        #[cfg(target_os = "macos")]
        {
            window = window.with_menu(main_menu)
        }
    }

    #[cfg(target_os = "macos")]
    {
        use dioxus_desktop::tao::platform::macos::WindowBuilderExtMacOS;

        window = window
            .with_has_shadow(true)
            .with_transparent(true)
            .with_fullsize_content_view(true)
            .with_titlebar_transparent(true)
            .with_title("")
    }

    #[cfg(not(target_os = "macos"))]
    {
        window = window.with_decorations(false).with_transparent(true);
    }
    window
}
