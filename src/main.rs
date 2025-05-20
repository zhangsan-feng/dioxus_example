
use dioxus::prelude::*;
use dioxus_desktop::{tao::dpi, Config, WindowBuilder};
use dioxus_desktop::tao;


#[derive(PartialEq, Clone)]
enum Page {
    Home,
    Counter,
    About,
}

fn App(cx: Scope) -> Element {
    let mut count = use_state(cx, || 0);
    let current_page = use_state(cx, || Page::Home);

    cx.render(rsx! {
        div {
            style: "display: flex;",
            // 左侧导航栏
            div {
                class: "sidebar",
                style: "width: 200px; background-color: #f0f0f0; padding: 20px; height: 90vh;",
                h2 { "导航菜单" }
                div {
                    style: "display: flex; flex-direction: column; gap: 10px;",
                    button {
                        onclick: move |_| current_page.set(Page::Home),
                        style: if *current_page == Page::Home { "background-color: #ddd;" } else { "" },
                        "首页"
                    }
                    button {
                        onclick: move |_| current_page.set(Page::Counter),
                        style: if *current_page == Page::Counter { "background-color: #ddd;" } else { "" },
                        "计数器"
                    }
                    button {
                        onclick: move |_| current_page.set(Page::About),
                        style: if *current_page == Page::About { "background-color: #ddd;" } else { "" },
                        "关于"
                    }
                }
            }
            // 右侧内容区
            div {
                style: "flex-grow: 1; padding: 20px;",
                match **current_page {
                    Page::Home => rsx! {
                        div {
                            h1 { "欢迎来到首页" }
                            table {
                                style: "width: 100%; border-collapse: collapse; margin-top: 20px;",
                                tr {
                                    th { "姓名" }
                                    th { "年龄" }
                                    th { "性别" }
                                }
                                tr {
                                    td { "张三" }
                                    td { "25" }
                                    td { "男" }
                                }
                                tr {
                                    td { "李四" }
                                    td { "30" }
                                    td { "女" }
                                }
                            }
                            input {
                                style: "width: 100%; padding: 10px; margin-bottom: 10px;",
                                placeholder: "请输入",
                                oninput: move |e| println!("{}", e.value),
                            }
                            p { "这是一个使用 Dioxus 构建的桌面应用示例。" }
                        }
                    },
                    Page::Counter => rsx! {
                        div {
                            style: "text-align: center;",
                            h1 { "计数器页面" }
                            p { "当前计数: {count}" }
                            button {
                                onclick: move |_| count += 1,
                                "增加"
                            }
                            button {
                                onclick: move |_| {
                                    if *count > 0 {
                                        count -= 1
                                    }
                                },
                                style: "margin-left: 10px;",
                                "减少"
                            }
                        }
                    },
                    Page::About => rsx! {
                        div {
                            h1 { "关于" }
                            p { "这是一个关于页面。" }
                            p { "使用 Dioxus 框架开发。" }
                        }
                    }
                }
            }
        }
    })
}



fn main() {
    let event_loop = tao::event_loop::EventLoop::new();
    let primary_monitor = event_loop.primary_monitor().unwrap(); 
    let screen_size = primary_monitor.size(); 

    let window_size = dpi::PhysicalSize::new(1000, 600); 
    let x = ((screen_size.width - window_size.width) / 2).try_into().unwrap();
    let y = ((screen_size.height - window_size.height) / 2).try_into().unwrap();

    let icon_bytes = include_bytes!("icons/title_icon.png");
    let img = image::load_from_memory(icon_bytes).expect("REASON").into_rgba8(); 
    let (width, height,rgb) = (img.width(), img.height(), img.into_raw());

    let window = WindowBuilder::new()
        .with_title("Dioxus Desktop App")
        .with_position(dpi::Position::Physical(dpi::PhysicalPosition::new(x,y)))
        .with_inner_size(dpi::Size::new(dpi::PhysicalSize::new(window_size.width,window_size.height)))
        .with_window_icon(Some(tao::window::Icon::from_rgba(rgb, width, height).expect("msg")))
        ;

    let desktop_config = Config::new().with_window(window);
    dioxus_desktop::launch_cfg(App, desktop_config);
}
