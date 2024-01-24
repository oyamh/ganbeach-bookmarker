'use strict';

// console.debug("check commands in background");
// console.debug(chrome.commands);

import init from "../wasm/background/background.js";
// import init from "../wasm/test/background.js";
init();

// chrome.commands.onCommand.addListener(async (command) => {
//     console.debug("pushed js shortcut key=", command);
// });

// console.debug("on message addlisatener")
// chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
//     console.debug("got message in background-script-inject");
//     console.debug({ message, sender, sendResponse });
//     if (sender.id != chrome.runtime.id) {
//         return;
//     }
//     if (!message.fromJS) {
//         return;
//     }
//     if (message.fromJS === "FailToImportWASM") {
//         const currentTab = sender.tab;
//         const tabId = currentTab.id;
//         console.debug({ currentTab });
//         chrome.action.setBadgeText({
//             text: "×",
//             tabId,
//         });
//         chrome.action.setBadgeTextColor({
//             color: "#FFFFFF",
//             tabId,
//         });
//         chrome.action.setBadgeBackgroundColor({
//             color: "#eb3737",
//             tabId,
//         });
//         return;
//     }
// })

// // const DBOpenRequest = window.indexedDB.open("toDoList", 4);

// // let db;

// // DBOpenRequest.onerror = event => {
// //     console.error(event);
// // }

// // DBOpenRequest.onsuccess = event => {
// //     db = DBOpenRequest.result;
// // }

// // DBOpenRequest.onupgradeneeded = e => {
// //     e.target.result.createObjectStore("store").add(BigInt(623677211081183232n), 1);
// //     e.target.onsuccess = e => {
// //         e.target.result.transaction("store").objectStore("store").get(1).onsuccess = e => { console.info("get ", e.target.result) };
// //     }
// // }

// // chrome.action.onClicked.addListener(() => {
// //     console.log("action clicked")
// //     chrome.notifications.create('TEST_NOTFICATION_ID 1', {
// //         type: 'basic',
// //         iconUrl: '../icons/book48.png',
// //         title: 'test notification title',
// //         message: 'test notification message',
// //         priority: 2
// //     });
// // });
