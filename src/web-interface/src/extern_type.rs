use js_sys::{Function, Promise};
use wasm_bindgen::{prelude::*, JsStatic};

// #[wasm_bindgen]
// extern "C" {
//     #[derive(Debug)]
//     pub type Window;

//     #[wasm_bindgen(js_name = window)]
//     pub static WINDOW: Window;

//     // #[wasm_bindgen(method)]
//     // pub fn open(this: &Window, url: String) -> Window;
// }

#[wasm_bindgen]
extern "C" {
    type Browser;
    #[wasm_bindgen(js_name = browser)]
    static BROWSER: Browser;
    #[wasm_bindgen(js_name = chrome)]
    static CHROME: Browser;

    // Manifest V2
    #[wasm_bindgen(method, getter, js_name = browserAction)]
    fn browser_action(this: &Browser) -> Action;
    // Manifest V3
    #[wasm_bindgen(method, getter, js_name = action)]
    fn action(this: &Browser) -> Action;

    #[wasm_bindgen(method, getter)]
    fn runtime(this: &Browser) -> Runtime;

    #[wasm_bindgen(method, getter)]
    fn windows(this: &Browser) -> Windows;

    #[wasm_bindgen(method, getter)]
    fn tabs(this: &Browser) -> Tabs;

    #[wasm_bindgen(method, getter)]
    fn commands(this: &Browser) -> Commands;

    #[wasm_bindgen(method, getter)]
    fn cookies(this: &Browser) -> Cookies;

    #[wasm_bindgen(method, getter)]
    fn notifications(this: &Browser) -> Notifications;

    #[wasm_bindgen(method, getter)]
    fn bookmarks(this: &Browser) -> Bookmarks;

    #[wasm_bindgen(method, getter)]
    fn history(this: &Browser) -> History;
}

// fn browser() -> &'static JsStatic<Browser> {
//     if cfg!(feature = "firefox") {
//         &BROWSER
//     } else if cfg!(feature = "chrome") {
//         &CHROME
//     } else {
//         &CHROME
//     }
// }

fn browser() -> &'static JsStatic<Browser> {
    #[cfg(feature = "firefox")]
    {
        &BROWSER
    }
    #[cfg(feature = "chrome")]
    {
        &CHROME
    }
    #[cfg(not(any(feature = "chrome", feature = "firefox")))]
    {
        &CHROME
    }
}

// #[cfg(feature = "firefox")]
// fn browser() -> &'static JsStatic<Browser> {
//     &BROWSER
// }
// #[cfg(feature = "chrome")]
// fn browser() -> &'static JsStatic<Browser> {
//     &CHROME
// }
// #[cfg(not(any(feature = "chrome", feature = "firefox")))]
// fn browser() -> &'static JsStatic<Browser> {
//     &BROWSER
// }

#[wasm_bindgen]
extern "C" {
    type Action;

    #[wasm_bindgen(method, getter, js_name = onClicked)]
    fn on_clicked(this: &Action) -> Event;

    #[wasm_bindgen(method, js_name = setBadgeText)]
    fn set_badge_text(this: &Action, message: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = setBadgeTextColor)]
    fn set_badge_text_color(this: &Action, message: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = setBadgeBackgroundColor)]
    fn set_badge_background_color(this: &Action, message: JsValue) -> Promise;
}

// fn browser_action() -> Action {
//     if cfg!(feature = "firefox") {
//         browser().browser_action()
//     } else if cfg!(feature = "chrome") {
//         browser().action()
//     } else {
//         browser().action()
//     }
// }

fn browser_action() -> Action {
    #[cfg(feature = "firefox")]
    {
        browser().browser_action()
    }
    #[cfg(feature = "chrome")]
    {
        browser().action()
    }
    #[cfg(not(any(feature = "chrome", feature = "firefox")))]
    {
        browser().action()
    }
}

// #[cfg(feature = "firefox")]
// fn browser_action() -> Action {
//     browser().browser_action()
// }
// #[cfg(feature = "chrome")]
// fn browser_action() -> Action {
//     browser().action()
// }
// #[cfg(not(any(feature = "chrome", feature = "firefox")))]
// fn browser_action() -> Action {
//     browser().action()
// }

pub fn on_clicked_toolbar(callback: &Function) {
    browser_action().on_clicked().add_listener(callback);
}

#[wasm_bindgen]
extern "C" {
    type Runtime;
    // pub static runtime: Runtime;

    #[wasm_bindgen(method, js_name = getURL)]
    fn get_url(this: &Runtime, path: &str) -> String;

    #[wasm_bindgen(method, getter, js_name = onMessage)]
    fn on_message(this: &Runtime) -> Event;

    #[wasm_bindgen(method, getter, js_name = onStartup)]
    fn on_startup(this: &Runtime) -> Event;

    /// send_message: contentスクリプト(clientside)からbackgroundスクリプト(backend)にmessageを送信する。
    #[wasm_bindgen(method, js_name = sendMessage)]
    fn send_message(this: &Runtime, message: JsValue) -> Promise;
    // pub fn send_message(this: &Runtime, message: &::wasm_bindgen::JsValue) -> Promise;
}

fn runtime() -> Runtime {
    browser().runtime()
}

pub fn get_url(path: &str) -> String {
    runtime().get_url(path)
}

pub fn send_browser_message(message: JsValue) -> Promise {
    runtime().send_message(message)
}

#[wasm_bindgen]
extern "C" {
    type Event;

    #[wasm_bindgen(method, js_name = addListener)]
    fn add_listener(this: &Event, callback: &Function);
}

pub fn on_browser_message(callback: &Function) {
    runtime().on_message().add_listener(callback);
}

pub fn on_startup(callback: &Function) {
    runtime().on_startup().add_listener(callback);
}

#[wasm_bindgen]
extern "C" {
    type Windows;

    #[wasm_bindgen(method, js_name = create)]
    fn create(this: &Windows, create_properties: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = get)]
    fn get(this: &Windows, window_id: i32) -> Promise;
}

fn windows() -> Windows {
    browser().windows()
}

pub fn create_window(create_properties: JsValue) -> Promise {
    windows().create(create_properties)
}

pub fn get_window(window_id: i32) -> Promise {
    windows().get(window_id)
}

#[wasm_bindgen]
extern "C" {
    type Tabs;
    // pub static tabs: Tabs;

    #[wasm_bindgen(method, js_name = create)]
    fn create(this: &Tabs, create_properties: JsValue) -> Promise;

    /// https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/tabs/sendMessage
    #[wasm_bindgen(method, js_name = sendMessage)]
    fn send_message(this: &Tabs, tab_id: i32, message: JsValue, options: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = query)]
    fn query(this: &Tabs, query_info: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = Tab)]
    fn tab(this: &Tabs) -> Tab;
}

fn tabs() -> Tabs {
    browser().tabs()
}

pub fn create_tab(create_properties: JsValue) -> Promise {
    tabs().create(create_properties)
}

pub fn send_tab_message(tab_id: i32, message: JsValue, options: JsValue) -> Promise {
    tabs().send_message(tab_id, message, options)
}

pub fn query_tab(query_info: JsValue) -> Promise {
    tabs().query(query_info)
}

#[wasm_bindgen]
extern "C" {
    type Tab;
    // pub static tab: Tab;

    #[wasm_bindgen(method, getter)]
    fn active(this: &Tab) -> bool;
    #[wasm_bindgen(method, getter)]
    fn id(this: &Tab) -> Option<i32>;
}

#[wasm_bindgen]
extern "C" {
    type Commands;

    #[wasm_bindgen(method, getter, js_name = onCommand)]
    fn on_command(this: &Commands) -> Event;
}

pub fn on_command(callback: &Function) {
    browser().commands().on_command().add_listener(callback);
}

#[wasm_bindgen]
extern "C" {
    type Cookies;
    // pub static cookies: Cookies;

    #[wasm_bindgen(method, js_name = get)]
    fn get(this: &Cookies, details: JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = getAll)]
    fn get_all(this: &Cookies, details: JsValue) -> JsValue;

    #[wasm_bindgen(method, js_name = remove)]
    fn remove(this: &Cookies, details: JsValue) -> Promise;

    #[wasm_bindgen(method, getter, js_name = onChanged)]
    fn on_changed(this: &Cookies) -> Event;
}

pub fn get_browser_cookie(details: JsValue) -> Promise {
    browser().cookies().get(details)
}

pub fn remove_browser_cookie(details: JsValue) -> Promise {
    browser().cookies().remove(details)
}

// pub fn on_change_cookie(callback: &Function) {
//     browser().cookies().on_changed().add_listener(callback);
// }

#[wasm_bindgen]
extern "C" {
    type Notifications;

    #[wasm_bindgen(method, js_name = create)]
    fn create(this: &Notifications, message: String, options: JsValue) -> Promise;

    // #[wasm_bindgen(method, js_name = get_all)]
    // fn get_all(this: &Notifications) -> Promise;

    #[wasm_bindgen(method, getter, js_name = onClicked)]
    fn on_clicked(this: &Notifications) -> Event;

    #[wasm_bindgen(method, getter, js_name = onClosed)]
    fn on_closed(this: &Notifications) -> Event;
}

fn notifications() -> Notifications {
    browser().notifications()
}

// pub fn create_notification(id: String, options: JsValue) -> Option<Promise> {
//     if cfg!(feature = "firefox") {
//         Some(notifications().create(id, options))
//     } else if cfg!(feature = "chrome") {
//         notifications().create(id, options);
//         None
//     } else {
//         notifications().create(id, options);
//         None
//     }
// }

pub fn create_notification(id: String, options: JsValue) -> Option<Promise> {
    #[cfg(feature = "firefox")]
    {
        Some(notifications().create(id, options))
    }
    #[cfg(feature = "chrome")]
    {
        let _promise = notifications().create(id, options);
        None
    }
    #[cfg(not(any(feature = "chrome", feature = "firefox")))]
    {
        let _promise = notifications().create(id, options);
        None
    }
}

// #[cfg(feature = "firefox")]
// pub fn create_notification(id: String, options: JsValue) -> Option<Promise> {
//     Some(notifications().create(id, options))
// }
// #[cfg(feature = "chrome")]
// pub fn create_notification(id: String, options: JsValue) -> Option<Promise> {
//     // Chrome notifiactions.create() returns undefined
//     let _promise = notifications().create(id, options);
//     None
// }
// #[cfg(not(any(feature = "chrome", feature = "firefox")))]
// pub fn create_notification(id: String, options: JsValue) -> Option<Promise> {
//     let _promise = notifications().create(id, options);
//     None
// }

// pub fn get_notifications() -> Promise {
//     notifications().get_all()
// }

// pub fn on_clicked_notification(callback: &Function) {
//     notifications().on_clicked().add_listener(callback);
// }

// pub fn on_closed_notification(callback: &Function) {
//     notifications().on_closed().add_listener(callback);
// }

// /// [BookmarkTreeNode](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks/BookmarkTreeNode)
// #[wasm_bindgen]
// extern "C" {
//     type BookmarkTreeNode;
//     #[wasm_bindgen(method, getter)]
//     fn id(this: &BookmarkTreeNode) -> String;
//     #[wasm_bindgen(method, getter, js_name = parentId)]
//     fn parent_id(this: &BookmarkTreeNode) -> Option<String>;
//     #[wasm_bindgen(method, getter)]
//     fn title(this: &BookmarkTreeNode) -> String;
// }

/// [bookmarks](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/API/bookmarks)
#[wasm_bindgen]
extern "C" {
    type Bookmarks;

    #[wasm_bindgen(method, js_name = create)]
    fn create(this: &Bookmarks, create_details: &JsValue) -> Promise;

    #[wasm_bindgen(method, js_name = search)]
    fn search(this: &Bookmarks, query: &JsValue) -> Promise;
}

fn bookmarks() -> Bookmarks {
    browser().bookmarks()
}

pub fn create_bookmark(create_details: &JsValue) -> Promise {
    bookmarks().create(create_details)
}

pub fn search_bookmark(query: &JsValue) -> Promise {
    bookmarks().search(query)
}

// #[wasm_bindgen]
// extern "C" {
//     #[wasm_bindgen(js_name = "setTimeout", catch)]
//     pub fn set_timeout(callback: &Function, timeout: i32) -> Result<JsValue, JsValue>;
// }

/// [history](https://developer.mozilla.org/ja/docs/Mozilla/Add-ons/WebExtensions/API/history)
#[wasm_bindgen]
extern "C" {
    type History;

    #[wasm_bindgen(method, getter, js_name = onVisited)]
    fn on_visited(this: &History) -> Event;

    #[wasm_bindgen(method, js_name = deleteUrl)]
    fn delete_url(this: &History, history_details: &JsValue) -> Promise;
}

fn history() -> History {
    browser().history()
}

pub fn delete_url(history_details: &JsValue) -> Promise {
    history().delete_url(history_details)
}

pub fn on_visited(callback: &Function) {
    history().on_visited().add_listener(callback);
}
