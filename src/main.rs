use dioxus::prelude::*;
use dioxus_desktop::tao;
use dioxus_desktop::Config;
use dioxus_desktop::LogicalSize;

use dioxus_desktop::tao::event_loop::EventLoopBuilder;
use dioxus_desktop::wry;
use muda::AboutMetadata;
use muda::Menu;
use muda::PredefinedMenuItem;
use muda::Submenu;
use tao::window::WindowBuilder;

use tracing_subscriber::filter::FilterFn;

use tracing_subscriber::{
    layer::{Layer, SubscriberExt},
    util::SubscriberInitExt,
};

#[cfg(target_os = "macos")]
use wry::application::platform::macos::WindowExtMacOS;
#[cfg(target_os = "linux")]
use wry::application::platform::unix::WindowExtUnix;
#[cfg(target_os = "windows")]
use wry::application::platform::windows::{EventLoopBuilderExtWindows, WindowExtWindows};

fn main() {
    let my_filter = FilterFn::new(|metadata| true);

    let my_layer = tracing_subscriber::fmt::layer();

    tracing_subscriber::registry()
        .with(my_layer.with_filter(my_filter))
        .init();

    // using Config::default fixes the crash on macos
    // not adding the `main_menu` in `webview_config()` also fixes the crash on macos
    dioxus_desktop::launch_cfg(app, /*Config::default())*/ webview_config())
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
    let main_menu = Menu::new();
    let app_menu = Submenu::new("Uplink", true);
    let edit_menu = Submenu::new("Edit", true);
    let window_menu = Submenu::new("Window", true);

    app_menu.append_items(&[
        &PredefinedMenuItem::about("Uplink".into(), Some(AboutMetadata::default())),
        &PredefinedMenuItem::quit(None),
    ]);
    // add native shortcuts to `edit_menu` menu
    // in macOS native item are required to get keyboard shortcut
    // to works correctly
    edit_menu.append_items(&[
        &PredefinedMenuItem::undo(None),
        &PredefinedMenuItem::redo(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::cut(None),
        &PredefinedMenuItem::copy(None),
        &PredefinedMenuItem::paste(None),
        &PredefinedMenuItem::select_all(None),
    ]);

    window_menu.append_items(&[
        &PredefinedMenuItem::minimize(None),
        //&PredefinedMenuItem::zoom(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::show_all(None),
        &PredefinedMenuItem::fullscreen(None),
        &PredefinedMenuItem::separator(),
        &PredefinedMenuItem::close_window(None),
    ]);

    main_menu.append(&app_menu);
    main_menu.append(&edit_menu);
    main_menu.append(&window_menu);

    /*let mut event_loop_builder = EventLoopBuilder::new();

    #[cfg(target_os = "windows")]
    {
        let menu_bar = main_menu.clone();
        event_loop_builder.with_msg_hook(move |msg| {
            use windows_sys::Win32::UI::WindowsAndMessaging::{TranslateAcceleratorW, MSG};
            unsafe {
                let msg = msg as *const MSG;
                let translated = TranslateAcceleratorW((*msg).hwnd, menu_bar.haccel(), msg);
                translated == 1
            }
        });
    }*/

    //let event_loop = event_loop_builder.build();

    let mut window = WindowBuilder::new()
        .with_title("uplink")
        .with_resizable(true)
        // We start the min inner size smaller because the prelude pages like unlock can be rendered much smaller.
        .with_min_inner_size(LogicalSize::new(300.0, 350.0));
    // .build(&event_loop)
    // .unwrap();

    if with_predefined_size {
        window = window.with_inner_size(LogicalSize::new(950.0, 600.0));
    }

    #[cfg(target_os = "windows")]
    {
        main_menu.init_for_hwnd(window.hwnd() as _);
    }
    #[cfg(target_os = "linux")]
    {
        main_menu.init_for_gtk_window(window.gtk_window(), window.default_vbox());
    }
    #[cfg(target_os = "macos")]
    {
        main_menu.init_for_nsapp();
    }

    if with_menu {
        #[cfg(target_os = "macos")]
        {
            //window = window.with_menu(main_menu)
        }
    }

    /*#[cfg(target_os = "macos")]
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
    }*/
    window
}
