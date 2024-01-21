import rust from "@wasm-tool/rollup-plugin-rust";
import { nodeResolve } from "@rollup/plugin-node-resolve";
import commonjs from "@rollup/plugin-commonjs";
import copy from "rollup-plugin-copy";
import del from "rollup-plugin-delete";

export default [
    {
        input: "./src/empty.js",
        output: {
            dir: "dummy",
            format: "es",
            sourcemap: false,
        },
        plugins: [
            del({ targets: "dist/*" }),
        ]
    },

    {
        input: {
            popup: "./src/popup/Cargo.toml",
            // message: "./src/message/Cargo.toml",
        },
        output: {
            dir: "dist/js",
            format: "es",
            sourcemap: true,
        },
        plugins: [
            nodeResolve(),
            commonjs(),
            rust({
                importHook: path => {
                    return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
                },
                serverPath: "js/",
                wasmBindgenArgs: ["--debug", "--keep-debug"],
                verbose: true,
            }),
        ]
    },

    {
        input: {
            background: "./src/background/Cargo.toml",
        },
        output: {
            dir: "dist/js",
            format: "iife",
            sourcemap: true,
        },
        plugins: [
            rust({
                importHook: path => {
                    return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
                },
                serverPath: "js/",
                wasmBindgenArgs: ["--debug", "--keep-debug"],
                verbose: true,
            }),
            copy({
                targets: [
                    { src: "static/*", dest: "dist" },
                ]
            }),
        ]
    },

    {
        input: {
            content: "./src/content/Cargo.toml",
        },
        output: {
            dir: "dist/js",
            format: "iife",
            sourcemap: true,
        },
        plugins: [
            rust({
                importHook: path => {
                    return "browser.runtime.getURL(" + JSON.stringify(path) + ")";
                },
                serverPath: "js/",
                wasmBindgenArgs: ["--debug", "--keep-debug"],
                verbose: true,
            }),
            copy({
                targets: [
                    { src: "static/*", dest: "dist" },
                ]
            }),
        ]
    }
]
