use dioxus::prelude::*;

// 定义页面枚举
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
    dioxus_desktop::launch(App);
}
