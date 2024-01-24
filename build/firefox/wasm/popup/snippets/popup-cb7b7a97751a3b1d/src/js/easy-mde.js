export class EasyMDEWrapper {
    constructor(baseElementId) {
        //console.log("EasyMDEWrapper created");
        const element = document.getElementById(baseElementId);
        this.easyMDE = new EasyMDE({
            element,
            status: false,
            tabSize: 4,
            styleSelectedText: false,
            spellChecker: false,
            nativeSpellcheck: false,//NOTE: スペルチェックを使わない。
            lineNumbers: true,
            minHeight: "126px",
            toolbar: ["heading", "code", "clean-block", "table", "horizontal-rule", "preview", "side-by-side", "fullscreen", "guide"],
        });
    }

    set_value(content) {
        //console.log("set_value", content);
        //this.easyMDE.codemirror.setValue(content);
        this.easyMDE.value(content);
    }

    get_value() {
        //console.log("get_value");
        //return this.easyMDE.codemirror.getValue();
        return this.easyMDE.value();
    }

    focus() {
        console.log("focus");
        this.easyMDE.codemirror.focus();
    }
}

//export class MyClass {
//    constructor() {
//        console.log("MyClass constructor");
//        this._str = "42";
//    }

//    get string() {
//        //console.log("get string", this._str);
//        return this._str;
//    }

//    set string(s) {
//        console.log("set string", s);
//        this._str = s;
//    }

//    render() {
//        //console.log("render", this._str);
//        return `My string is: ${this._str}`;
//    }
//}
