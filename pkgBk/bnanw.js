
let wasm;

const heap = new Array(32).fill(undefined);

heap.push(undefined, null, true, false);

function getObject(idx) { return heap[idx]; }

let heap_next = heap.length;

function dropObject(idx) {
    if (idx < 36) return;
    heap[idx] = heap_next;
    heap_next = idx;
}

function takeObject(idx) {
    const ret = getObject(idx);
    dropObject(idx);
    return ret;
}

let cachedTextDecoder = new TextDecoder('utf-8', { ignoreBOM: true, fatal: true });

cachedTextDecoder.decode();

let cachegetUint8Memory0 = null;
function getUint8Memory0() {
    if (cachegetUint8Memory0 === null || cachegetUint8Memory0.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory0 = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory0;
}

function getStringFromWasm0(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory0().subarray(ptr, ptr + len));
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function debugString(val) {
    // primitive types
    const type = typeof val;
    if (type == 'number' || type == 'boolean' || val == null) {
        return `${val}`;
    }
    if (type == 'string') {
        return `"${val}"`;
    }
    if (type == 'symbol') {
        const description = val.description;
        if (description == null) {
            return 'Symbol';
        } else {
            return `Symbol(${description})`;
        }
    }
    if (type == 'function') {
        const name = val.name;
        if (typeof name == 'string' && name.length > 0) {
            return `Function(${name})`;
        } else {
            return 'Function';
        }
    }
    // objects
    if (Array.isArray(val)) {
        const length = val.length;
        let debug = '[';
        if (length > 0) {
            debug += debugString(val[0]);
        }
        for (let i = 1; i < length; i++) {
            debug += ', ' + debugString(val[i]);
        }
        debug += ']';
        return debug;
    }
    // Test for built-in
    const builtInMatches = /\[object ([^\]]+)\]/.exec(toString.call(val));
    let className;
    if (builtInMatches.length > 1) {
        className = builtInMatches[1];
    } else {
        // Failed to match the standard '[object ClassName]'
        return toString.call(val);
    }
    if (className == 'Object') {
        // we're a user defined class or Object
        // JSON.stringify avoids problems with cycles, and is generally much
        // easier than looping through ownProperties of `val`.
        try {
            return 'Object(' + JSON.stringify(val) + ')';
        } catch (_) {
            return 'Object';
        }
    }
    // errors
    if (val instanceof Error) {
        return `${val.name}: ${val.message}\n${val.stack}`;
    }
    // TODO we could test for more things here, like `Set`s and `Map`s.
    return className;
}

let WASM_VECTOR_LEN = 0;

let cachedTextEncoder = new TextEncoder('utf-8');

const encodeString = (typeof cachedTextEncoder.encodeInto === 'function'
    ? function (arg, view) {
        return cachedTextEncoder.encodeInto(arg, view);
    }
    : function (arg, view) {
        const buf = cachedTextEncoder.encode(arg);
        view.set(buf);
        return {
            read: arg.length,
            written: buf.length
        };
    });

function passStringToWasm0(arg, malloc, realloc) {

    if (realloc === undefined) {
        const buf = cachedTextEncoder.encode(arg);
        const ptr = malloc(buf.length);
        getUint8Memory0().subarray(ptr, ptr + buf.length).set(buf);
        WASM_VECTOR_LEN = buf.length;
        return ptr;
    }

    let len = arg.length;
    let ptr = malloc(len);

    const mem = getUint8Memory0();

    let offset = 0;

    for (; offset < len; offset++) {
        const code = arg.charCodeAt(offset);
        if (code > 0x7F) break;
        mem[ptr + offset] = code;
    }

    if (offset !== len) {
        if (offset !== 0) {
            arg = arg.slice(offset);
        }
        ptr = realloc(ptr, len, len = offset + arg.length * 3);
        const view = getUint8Memory0().subarray(ptr + offset, ptr + len);
        const ret = encodeString(arg, view);

        offset += ret.written;
    }

    WASM_VECTOR_LEN = offset;
    return ptr;
}

let cachegetInt32Memory0 = null;
function getInt32Memory0() {
    if (cachegetInt32Memory0 === null || cachegetInt32Memory0.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory0 = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory0;
}

function makeMutClosure(arg0, arg1, dtor, f) {
    const state = { a: arg0, b: arg1, cnt: 1, dtor };
    const real = (...args) => {
        // First up with a closure we increment the internal reference
        // count. This ensures that the Rust closure environment won't
        // be deallocated while we're invoking it.
        state.cnt++;
        const a = state.a;
        state.a = 0;
        try {
            return f(a, state.b, ...args);
        } finally {
            if (--state.cnt === 0) {
                wasm.__wbindgen_export_2.get(state.dtor)(a, state.b);

            } else {
                state.a = a;
            }
        }
    };
    real.original = state;

    return real;
}
function __wbg_adapter_18(arg0, arg1) {
    wasm._dyn_core__ops__function__FnMut_____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__h6a3f6fb25b123bd8(arg0, arg1);
}

function __wbg_adapter_21(arg0, arg1, arg2) {
    wasm._dyn_core__ops__function__FnMut__A____Output___R_as_wasm_bindgen__closure__WasmClosure___describe__invoke__hf6b2e8a9d37b6d3b(arg0, arg1, addHeapObject(arg2));
}

/**
* 起動時処理
*/
export function main() {
    wasm.main();
}

/**
* 疎通確認
*
* # 引数
* input
*
* # 戻り値
* - input + 1
* @param {number} input
* @returns {number}
*/
export function ping(input) {
    var ret = wasm.ping(input);
    return ret;
}

/**
* Googleフォントロード処理
*
* # 引数
* なし
*
* # 戻り値
* なし
*/
export function load_font() {
    wasm.load_font();
}

/**
* 文書をセットする
*
* # 引数
* ## id
* ## title
* ## vertical
* - 1 : 横書き
* - 2 : 縦書き
* ## font_size
* ## current
*
* # 戻り値
* なし
* @param {number} id
* @param {string} title
* @param {number} vertical
* @param {number} font_size
* @param {number} current
*/
export function set_doc(id, title, vertical, font_size, current) {
    var ptr0 = passStringToWasm0(title, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.set_doc(id, ptr0, len0, vertical, font_size, current);
}

/**
* 文書の行をセットする
*
* # 引数
* ## seq
* ## text
*
* # 戻り値
* なし
* @param {number} seq
* @param {string} text
*/
export function set_source(seq, text) {
    var ptr0 = passStringToWasm0(text, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
    var len0 = WASM_VECTOR_LEN;
    wasm.set_source(seq, ptr0, len0);
}

/**
* 文書ツリーを生成する
*
* # 引数
* なし
*
* # 戻り値
* なし
*/
export function build_tree() {
    wasm.build_tree();
}

/**
* 文書を表示する
*
* # 引数
* ## width
* - キャンバスの幅
* ## height
* - キャンバスの高さ
* ## is_dark
* - true: ダークモード
* ## is_android
* - true: Android
*
* # 戻り値
* なし
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
* @param {boolean} is_android
*/
export function draw_doc(width, height, is_dark, is_android) {
    wasm.draw_doc(width, height, is_dark, is_android);
}

/**
* キャンバスサイズ変更
*
* # 引数
* ## width
* - キャンバスの幅
* ## height
* - キャンバスの高さ
* ## is_dark
* - true: ダークモード
*
* # 戻り値
* なし
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
*/
export function resize(width, height, is_dark) {
    wasm.resize(width, height, is_dark);
}

/**
* タブを切り替える
*
* # 引数
* ## tab
* - 0: 本文
* - 1: 目次
* - 2: Box
* - 3: 白板
*
* # 戻り値
* なし
* @param {number} tab
* @param {number} width
* @param {number} height
* @param {boolean} is_dark
* @returns {number}
*/
export function tab_change(tab, width, height, is_dark) {
    var ret = wasm.tab_change(tab, width, height, is_dark);
    return ret;
}

/**
* 現在のセクションを返す
*
* # 引数
* なし
*
* # 戻り値
* セクション
* @returns {number}
*/
export function get_section() {
    var ret = wasm.get_section();
    return ret;
}

/**
* タッチ開始
*
* # 引数
* ## x
* ## y
*
* # 戻り値
* なし
* @param {number} x
* @param {number} y
*/
export function touch_start(x, y) {
    wasm.touch_start(x, y);
}

/**
* タッチを移動する
*
* # 引数
* ## x
* ## y
*
* # 戻り値
* なし
* @param {number} x
* @param {number} y
*/
export function touch_move(x, y) {
    wasm.touch_move(x, y);
}

/**
* タッチ終了
*
* # 引数
* なし
*
* # 戻り値
* - -2 : 正常終了
* - -1 : Top選択
* - 0以上 : セクション選択
* - それ以外 : 異常終了
* @returns {number}
*/
export function touch_end() {
    var ret = wasm.touch_end();
    return ret;
}

/**
* 黒塗りモードを変更する
*
* # 引数
* ## black
*
* # 戻り値
* なし
* @param {boolean} black
*/
export function mode_change(black) {
    wasm.mode_change(black);
}

/**
* ツールボタンの操作
*
* # 引数
*
* ## step
* - 1 : 1区切り進む
* - 2 : 1区切り戻る
* - 3 : 1単語進む
* - 4 : 末尾に進む
* - 5 : 先頭に戻る
* - 6 : 次の段・節に進む
* - 7 : 前の段・節に戻る
*
* # 戻り値
* なし
* @param {number} step
*/
export function tool_func(step) {
    wasm.tool_func(step);
}

/**
* 白板・戻る
*
* # 引数
*
* # 戻り値
* なし
*/
export function stroke_back() {
    wasm.stroke_back();
}

/**
* 白板・消去
*
* # 引数
*
* # 戻り値
* なし
*/
export function stroke_clear() {
    wasm.stroke_clear();
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

function handleError(f, args) {
    try {
        return f.apply(this, args);
    } catch (e) {
        wasm.__wbindgen_exn_store(addHeapObject(e));
    }
}

async function load(module, imports) {
    if (typeof Response === 'function' && module instanceof Response) {
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            try {
                return await WebAssembly.instantiateStreaming(module, imports);

            } catch (e) {
                if (module.headers.get('Content-Type') != 'application/wasm') {
                    console.warn("`WebAssembly.instantiateStreaming` failed because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);

                } else {
                    throw e;
                }
            }
        }

        const bytes = await module.arrayBuffer();
        return await WebAssembly.instantiate(bytes, imports);

    } else {
        const instance = await WebAssembly.instantiate(module, imports);

        if (instance instanceof WebAssembly.Instance) {
            return { instance, module };

        } else {
            return instance;
        }
    }
}

async function init(input) {
    /*
    if (typeof input === 'undefined') {
        input = new URL('bnanw_bg.wasm', import.meta.url);
    }
    */
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_object_drop_ref = function (arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_string_new = function (arg0, arg1) {
        var ret = getStringFromWasm0(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_cb_drop = function (arg0) {
        const obj = takeObject(arg0).original;
        if (obj.cnt-- == 1) {
            obj.a = 0;
            return true;
        }
        var ret = false;
        return ret;
    };
    imports.wbg.__wbg_setTimeout_2504348a5dfd69b5 = function (arg0, arg1) {
        var ret = setTimeout(getObject(arg0), arg1 >>> 0);
        return ret;
    };
    imports.wbg.__wbg_instanceof_Window_c4b70662a0d2c5ec = function (arg0) {
        var ret = getObject(arg0) instanceof Window;
        return ret;
    };
    imports.wbg.__wbg_document_1c64944725c0d81d = function (arg0) {
        var ret = getObject(arg0).document;
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_fonts_7ba88c46c0be3d78 = function (arg0) {
        var ret = getObject(arg0).fonts;
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_getElementById_f3e94458ce77f0d0 = function (arg0, arg1, arg2) {
        var ret = getObject(arg0).getElementById(getStringFromWasm0(arg1, arg2));
        return isLikeNone(ret) ? 0 : addHeapObject(ret);
    };
    imports.wbg.__wbg_newwithstr_0a93bf0ab33337f9 = function () {
        return handleError(function (arg0, arg1, arg2, arg3) {
            var ret = new FontFace(getStringFromWasm0(arg0, arg1), getStringFromWasm0(arg2, arg3));
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_load_a639bcf538949467 = function () {
        return handleError(function (arg0) {
            var ret = getObject(arg0).load();
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_add_3bb7ad43e1c50cc9 = function () {
        return handleError(function (arg0, arg1) {
            getObject(arg0).add(getObject(arg1));
        }, arguments)
    };
    imports.wbg.__wbg_instanceof_HtmlCanvasElement_25d964a0dde6717e = function (arg0) {
        var ret = getObject(arg0) instanceof HTMLCanvasElement;
        return ret;
    };
    imports.wbg.__wbg_width_555f63ab09ba7d3f = function (arg0) {
        var ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_setwidth_c1a7061891b71f25 = function (arg0, arg1) {
        getObject(arg0).width = arg1 >>> 0;
    };
    imports.wbg.__wbg_height_7153faec70fbaf7b = function (arg0) {
        var ret = getObject(arg0).height;
        return ret;
    };
    imports.wbg.__wbg_setheight_88894b05710ff752 = function (arg0, arg1) {
        getObject(arg0).height = arg1 >>> 0;
    };
    imports.wbg.__wbg_getContext_f701d0231ae22393 = function () {
        return handleError(function (arg0, arg1, arg2) {
            var ret = getObject(arg0).getContext(getStringFromWasm0(arg1, arg2));
            return isLikeNone(ret) ? 0 : addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_log_3445347661d4505e = function (arg0) {
        console.log(getObject(arg0));
    };
    imports.wbg.__wbg_instanceof_CanvasRenderingContext2d_3abbe7ec7af32cae = function (arg0) {
        var ret = getObject(arg0) instanceof CanvasRenderingContext2D;
        return ret;
    };
    imports.wbg.__wbg_setstrokeStyle_947bd4c26c94673f = function (arg0, arg1) {
        getObject(arg0).strokeStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setfillStyle_528a6a267c863ae7 = function (arg0, arg1) {
        getObject(arg0).fillStyle = getObject(arg1);
    };
    imports.wbg.__wbg_setlineWidth_3221b7818c00ed48 = function (arg0, arg1) {
        getObject(arg0).lineWidth = arg1;
    };
    imports.wbg.__wbg_setfont_884816cc1b46ae3f = function (arg0, arg1, arg2) {
        getObject(arg0).font = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settextAlign_1891d6f4d7f9b9a3 = function (arg0, arg1, arg2) {
        getObject(arg0).textAlign = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_settextBaseline_3b90a2129ee3dead = function (arg0, arg1, arg2) {
        getObject(arg0).textBaseline = getStringFromWasm0(arg1, arg2);
    };
    imports.wbg.__wbg_beginPath_733d5a9e3e769d24 = function (arg0) {
        getObject(arg0).beginPath();
    };
    imports.wbg.__wbg_fill_dc4e97599365a189 = function (arg0) {
        getObject(arg0).fill();
    };
    imports.wbg.__wbg_stroke_7cdcdf3d07636d76 = function (arg0) {
        getObject(arg0).stroke();
    };
    imports.wbg.__wbg_arc_bdfc39ad6001708b = function () {
        return handleError(function (arg0, arg1, arg2, arg3, arg4, arg5) {
            getObject(arg0).arc(arg1, arg2, arg3, arg4, arg5);
        }, arguments)
    };
    imports.wbg.__wbg_lineTo_fde385edd804f315 = function (arg0, arg1, arg2) {
        getObject(arg0).lineTo(arg1, arg2);
    };
    imports.wbg.__wbg_moveTo_18ace182fe51d75d = function (arg0, arg1, arg2) {
        getObject(arg0).moveTo(arg1, arg2);
    };
    imports.wbg.__wbg_fillRect_10e42dc7a5e8cccd = function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).fillRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_strokeRect_74c84ef5e5ba1eaa = function (arg0, arg1, arg2, arg3, arg4) {
        getObject(arg0).strokeRect(arg1, arg2, arg3, arg4);
    };
    imports.wbg.__wbg_fillText_25221e9cc35a1850 = function () {
        return handleError(function (arg0, arg1, arg2, arg3, arg4) {
            getObject(arg0).fillText(getStringFromWasm0(arg1, arg2), arg3, arg4);
        }, arguments)
    };
    imports.wbg.__wbg_measureText_646aac3696f5cad5 = function () {
        return handleError(function (arg0, arg1, arg2) {
            var ret = getObject(arg0).measureText(getStringFromWasm0(arg1, arg2));
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_rotate_360dbdd13dc1b620 = function () {
        return handleError(function (arg0, arg1) {
            getObject(arg0).rotate(arg1);
        }, arguments)
    };
    imports.wbg.__wbg_width_4dd0ad3fb763f881 = function (arg0) {
        var ret = getObject(arg0).width;
        return ret;
    };
    imports.wbg.__wbg_newnoargs_be86524d73f67598 = function (arg0, arg1) {
        var ret = new Function(getStringFromWasm0(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_call_888d259a5fefc347 = function () {
        return handleError(function (arg0, arg1) {
            var ret = getObject(arg0).call(getObject(arg1));
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbindgen_object_clone_ref = function (arg0) {
        var ret = getObject(arg0);
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_now_af172eabe2e041ad = function () {
        var ret = Date.now();
        return ret;
    };
    imports.wbg.__wbg_resolve_d23068002f584f22 = function (arg0) {
        var ret = Promise.resolve(getObject(arg0));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_2fcac196782070cc = function (arg0, arg1) {
        var ret = getObject(arg0).then(getObject(arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_then_8c2d62e8ae5978f7 = function (arg0, arg1, arg2) {
        var ret = getObject(arg0).then(getObject(arg1), getObject(arg2));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_self_c6fbdfc2918d5e58 = function () {
        return handleError(function () {
            var ret = self.self;
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_window_baec038b5ab35c54 = function () {
        return handleError(function () {
            var ret = window.window;
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_globalThis_3f735a5746d41fbd = function () {
        return handleError(function () {
            var ret = globalThis.globalThis;
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbg_global_1bc0b39582740e95 = function () {
        return handleError(function () {
            var ret = global.global;
            return addHeapObject(ret);
        }, arguments)
    };
    imports.wbg.__wbindgen_is_undefined = function (arg0) {
        var ret = getObject(arg0) === undefined;
        return ret;
    };
    imports.wbg.__wbindgen_debug_string = function (arg0, arg1) {
        var ret = debugString(getObject(arg1));
        var ptr0 = passStringToWasm0(ret, wasm.__wbindgen_malloc, wasm.__wbindgen_realloc);
        var len0 = WASM_VECTOR_LEN;
        getInt32Memory0()[arg0 / 4 + 1] = len0;
        getInt32Memory0()[arg0 / 4 + 0] = ptr0;
    };
    imports.wbg.__wbindgen_throw = function (arg0, arg1) {
        throw new Error(getStringFromWasm0(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function (arg0) {
        throw takeObject(arg0);
    };
    imports.wbg.__wbindgen_closure_wrapper103 = function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 21, __wbg_adapter_18);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_closure_wrapper217 = function (arg0, arg1, arg2) {
        var ret = makeMutClosure(arg0, arg1, 49, __wbg_adapter_21);
        return addHeapObject(ret);
    };

    /*
    if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
        input = fetch(input);
    }

    const { instance, module } = await load(await input, imports);
    */
    const bin = await (await fetch('assets/pkg/bnanw_bg.wasm')).arrayBuffer();
    const { instance, module } = await WebAssembly.instantiate(bin, imports);

    wasm = instance.exports;
    init.__wbindgen_wasm_module = module;
    wasm.__wbindgen_start();
    return wasm;
}

export default init;
