# Ganbeach Bookmarker

## Features
- Browser extension to add bookmarks to Ganbeach.
- You can add tags and annotation.

## Installation

### Chrome
Install via [Chrome web store]() (Comming soon)

### Firefox
Install via [Firefox add-ons]() (Comming soon)

### Install Locally

#### Chrome
1. Go to [the extensions page](chrome://extensions). -> `chrome://extensions`
2. Enable Developer Mode.
3. Click on `Load unpacked` button
4. Browse the `ganbeach-bookmarker/dist/chrome` directory and select it.

#### Firefox
1. Go to [the addons](about:debugging#addons). -> `about:debugging#addons`
2. Click the `Load Temporary Add-On` button,
3. Browse the `ganbeach-bookmarker/dist/firefox/manifest.json` file and select it.

### Build from source
1. `git clone `
2. Run build script in `ganbeach-bookmarker`.
- Chrome
    ```bash
    $ ./script/release/chrome.sh
    ```
- Firefox
    ```bash
    $ ./script/release/firefox.sh
    ```

## Usage
This is just a bookmarker.
1. Click the icon on the toolbar to open the bookmark window.
2. Enter any Name, Folder, Tags, Annotation.
3. Click SEND button.

### Shortcut
- `Ctrl + Shift + L`: Open bookmark window

#### Change shortcut

##### Chrome
Go to [the shortcuts page](chrome://extensions/shortcuts). -> `chrome://extensions/shortcuts`

##### Firefox
1. Go to [Add-ons and themes](about:addons). -> `about:addons`
2. Select Extensions.
3. Click the Tools for all add-ons cogwheel.
4. Click Manage Extension Shortcuts in the menu.

## Permissions
This extension requires some permissions.
#### activeTab
- Required to get the information on the page needed to add a bookmark.
#### tabs
- Required to open a tab that assist you in logging in to Ganbeach.
#### cookies
- Required to securely store Refresh Token.
#### bookmarks
- Required to add the same data to the browser bookmarks.
#### notifications
- Required to notify errors.
#### history
- Required to remove the annoying history of the window for adding bookmarks.

## Tech Stack
- [Rust Programming Language](https://www.rust-lang.org/)
- [Yew](https://yew.rs/)
- [Connect](https://connectrpc.com/)
- [CodeMirror](https://codemirror.net/)

## License
This project is licensed under the [MIT License](https://opensource.org/license/mit/). Feel free to edit and distribute this template as you like.

See LICENSE for more information.
