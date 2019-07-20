let wasm;

let cachegetInt32Memory = null;
function getInt32Memory() {
    if (cachegetInt32Memory === null || cachegetInt32Memory.buffer !== wasm.memory.buffer) {
        cachegetInt32Memory = new Int32Array(wasm.memory.buffer);
    }
    return cachegetInt32Memory;
}

let cachegetUint8Memory = null;
function getUint8Memory() {
    if (cachegetUint8Memory === null || cachegetUint8Memory.buffer !== wasm.memory.buffer) {
        cachegetUint8Memory = new Uint8Array(wasm.memory.buffer);
    }
    return cachegetUint8Memory;
}

function getArrayU8FromWasm(ptr, len) {
    return getUint8Memory().subarray(ptr / 1, ptr / 1 + len);
}

function isLikeNone(x) {
    return x === undefined || x === null;
}

let cachegetUint32Memory = null;
function getUint32Memory() {
    if (cachegetUint32Memory === null || cachegetUint32Memory.buffer !== wasm.memory.buffer) {
        cachegetUint32Memory = new Uint32Array(wasm.memory.buffer);
    }
    return cachegetUint32Memory;
}

const heap = new Array(32);

heap.fill(undefined);

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

function getArrayJsValueFromWasm(ptr, len) {
    const mem = getUint32Memory();
    const slice = mem.subarray(ptr / 4, ptr / 4 + len);
    const result = [];
    for (let i = 0; i < slice.length; i++) {
        result.push(takeObject(slice[i]));
    }
    return result;
}

function _assertClass(instance, klass) {
    if (!(instance instanceof klass)) {
        throw new Error(`expected instance of ${klass.name}`);
    }
    return instance.ptr;
}

let WASM_VECTOR_LEN = 0;

function passArray8ToWasm(arg) {
    const ptr = wasm.__wbindgen_malloc(arg.length * 1);
    getUint8Memory().set(arg, ptr / 1);
    WASM_VECTOR_LEN = arg.length;
    return ptr;
}

function addHeapObject(obj) {
    if (heap_next === heap.length) heap.push(heap.length + 1);
    const idx = heap_next;
    heap_next = heap[idx];

    heap[idx] = obj;
    return idx;
}

function passArrayJsValueToWasm(array) {
    const ptr = wasm.__wbindgen_malloc(array.length * 4);
    const mem = getUint32Memory();
    for (let i = 0; i < array.length; i++) {
        mem[ptr / 4 + i] = addHeapObject(array[i]);
    }
    WASM_VECTOR_LEN = array.length;
    return ptr;
}

let cachedTextDecoder = new TextDecoder('utf-8');

function getStringFromWasm(ptr, len) {
    return cachedTextDecoder.decode(getUint8Memory().subarray(ptr, ptr + len));
}
/**
*/
const Token = Object.freeze({ Player1:0,Player2:1, });
/**
*/
class Game {

    static __wrap(ptr) {
        const obj = Object.create(Game.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_game_free(ptr);
    }
    /**
    * @returns {boolean}
    */
    over() {
        const ret = wasm.game_over(this.ptr);
        return ret !== 0;
    }
    /**
    * @returns {number}
    */
    current_player() {
        const ret = wasm.game_current_player(this.ptr);
        return ret;
    }
    /**
    * @returns {number}
    */
    cols() {
        const ret = wasm.game_cols(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {number}
    */
    rows() {
        const ret = wasm.game_rows(this.ptr);
        return ret >>> 0;
    }
    /**
    * @returns {Uint8Array}
    */
    state() {
        const retptr = 8;
        const ret = wasm.game_state(retptr, this.ptr);
        const memi32 = getInt32Memory();
        const v0 = getArrayU8FromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 1);
        return v0;
    }
    /**
    * @param {number | undefined} cols
    * @param {number | undefined} rows
    * @param {number | undefined} win_length
    * @returns {Game}
    */
    constructor(cols, rows, win_length) {
        const ret = wasm.game_ctor(!isLikeNone(cols), isLikeNone(cols) ? 0 : cols, !isLikeNone(rows), isLikeNone(rows) ? 0 : rows, !isLikeNone(win_length), isLikeNone(win_length) ? 0 : win_length);
        return Game.__wrap(ret);
    }
    /**
    * @returns {any[]}
    */
    valid_moves() {
        const retptr = 8;
        const ret = wasm.game_valid_moves(retptr, this.ptr);
        const memi32 = getInt32Memory();
        const v0 = getArrayJsValueFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 4);
        return v0;
    }
    /**
    * @returns {number}
    */
    winner() {
        const ret = wasm.game_winner(this.ptr);
        return ret === 2 ? undefined : ret;
    }
    /**
    * @returns {any[]}
    */
    winner_cells() {
        const retptr = 8;
        const ret = wasm.game_winner_cells(retptr, this.ptr);
        const memi32 = getInt32Memory();
        let v0;
        if (memi32[retptr / 4 + 0] !== 0) {
            v0 = getArrayJsValueFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
            wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 4);
        }
        return v0;
    }
    /**
    * @param {number} col
    * @returns {number}
    */
    drop(col) {
        const ret = wasm.game_drop(this.ptr, col);
        return ret >>> 0;
    }
    /**
    * @returns {any}
    */
    board() {
        const ret = wasm.game_board(this.ptr);
        return takeObject(ret);
    }
}
/**
*/
class MCTS {

    static __wrap(ptr) {
        const obj = Object.create(MCTS.prototype);
        obj.ptr = ptr;

        return obj;
    }

    free() {
        const ptr = this.ptr;
        this.ptr = 0;

        wasm.__wbg_mcts_free(ptr);
    }
    /**
    * @returns {MCTS}
    */
    constructor() {
        const ret = wasm.mcts_new();
        return MCTS.__wrap(ret);
    }
    /**
    * @param {Game} game
    * @returns {any[]}
    */
    simulate(game) {
        const retptr = 8;
        _assertClass(game, Game);
        const ret = wasm.mcts_simulate(retptr, this.ptr, game.ptr);
        const memi32 = getInt32Memory();
        const v0 = getArrayJsValueFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 4);
        return v0;
    }
    /**
    * @param {Game} game
    * @param {number} duration
    * @returns {any[]}
    */
    think(game, duration) {
        const retptr = 8;
        _assertClass(game, Game);
        const ret = wasm.mcts_think(retptr, this.ptr, game.ptr, duration);
        const memi32 = getInt32Memory();
        const v0 = getArrayJsValueFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 4);
        return v0;
    }
    /**
    * @param {Uint8Array} state
    * @param {any[]} moves
    * @returns {any[]}
    */
    move_weights(state, moves) {
        const retptr = 8;
        const ret = wasm.mcts_move_weights(retptr, this.ptr, passArray8ToWasm(state), WASM_VECTOR_LEN, passArrayJsValueToWasm(moves), WASM_VECTOR_LEN);
        const memi32 = getInt32Memory();
        const v0 = getArrayJsValueFromWasm(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1]).slice();
        wasm.__wbindgen_free(memi32[retptr / 4 + 0], memi32[retptr / 4 + 1] * 4);
        return v0;
    }
}

function init(module) {
    // if (typeof module === 'undefined') {
    //     module = import.meta.url.replace(/\.js$/, '_bg.wasm');
    // }
    let result;
    const imports = {};
    imports.wbg = {};
    imports.wbg.__wbindgen_number_new = function(arg0) {
        const ret = arg0;
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_string_new = function(arg0, arg1) {
        const ret = getStringFromWasm(arg0, arg1);
        return addHeapObject(ret);
    };
    imports.wbg.__wbindgen_json_parse = function(arg0, arg1) {
        const ret = JSON.parse(getStringFromWasm(arg0, arg1));
        return addHeapObject(ret);
    };
    imports.wbg.__wbg_random_3d74d328ba8726a5 = function() {
        const ret = Math.random();
        return ret;
    };
    imports.wbg.__wbg_now_905b1ca2d46dea4e = function() {
        const ret = Date.now();
        return ret;
    };
    imports.wbg.__wbindgen_object_drop_ref = function(arg0) {
        takeObject(arg0);
    };
    imports.wbg.__wbindgen_number_get = function(arg0, arg1) {
        const obj = getObject(arg0);
        if (typeof(obj) === 'number') return obj;
        getUint8Memory()[arg1] = 1;
        const ret = 0;
        return ret;
    };
    imports.wbg.__wbindgen_throw = function(arg0, arg1) {
        throw new Error(getStringFromWasm(arg0, arg1));
    };
    imports.wbg.__wbindgen_rethrow = function(arg0) {
        throw takeObject(arg0);
    };

    if (module instanceof URL || typeof module === 'string' || module instanceof Request) {

        const response = fetch(module);
        if (typeof WebAssembly.instantiateStreaming === 'function') {
            result = WebAssembly.instantiateStreaming(response, imports)
            .catch(e => {
                console.warn("`WebAssembly.instantiateStreaming` failed. Assuming this is because your server does not serve wasm with `application/wasm` MIME type. Falling back to `WebAssembly.instantiate` which is slower. Original error:\n", e);
                return response
                .then(r => r.arrayBuffer())
                .then(bytes => WebAssembly.instantiate(bytes, imports));
            });
        } else {
            result = response
            .then(r => r.arrayBuffer())
            .then(bytes => WebAssembly.instantiate(bytes, imports));
        }
    } else {

        result = WebAssembly.instantiate(module, imports)
        .then(result => {
            if (result instanceof WebAssembly.Instance) {
                return { instance: result, module };
            } else {
                return result;
            }
        });
    }
    return result.then(({instance, module}) => {
        wasm = instance.exports;
        init.__wbindgen_wasm_module = module;

        return wasm;
    });
}

const games = {};

function randomString() {
    return Math.random().toString().slice(2);
}

const functions = {
    newGame(opts) {
        const { cols, rows, win_length } = opts || {};
        let id;
        while ((id = randomString()) && id in games) {}
        const game = games[id] = new Game(cols, rows, win_length);

        return {
            id,
            cols: game.cols(),
            rows: game.rows(),
            moves: game.valid_moves(),
            board: game.board(),
        };
    },

    freeGame({ gameId }) {
        games[gameId].free();
        delete games[gameId];
    },
    
    drop({ gameId, column }) {
        if (!(gameId in games)) {
            throw new Error('game not found');
        }
    
        const game = games[gameId];
        const row = game.drop(column);
        return {
            cell: [column, row],
            over: game.over(),
            cells: game.winner_cells(),
            moves: game.valid_moves(),
            board: game.board(),
        }
    },

    think({ gameId, duration = 1000 }) {
        if (!(gameId in games)) {
            throw new Error('game not found');
        }

        const game = games[gameId];
        self.mcts.think(game, duration);

        const moves = game.valid_moves();
        const weights = self.mcts.move_weights(game.state(), moves);
        return moves.map((move, i) => [move, weights[i]]);
    }
};

onmessage = async (event) => {
    const { id, name, payload } = event.data;

    try {
        postMessage({ id, result: await functions[name](payload) });
    } catch (error) {
        postMessage({ id, error });
    }
};

init('mcts_bg.wasm').then(() => (self.mcts = new MCTS(), postMessage({ ready: true })), (error) => postMessage({ error }));
//# sourceMappingURL=mcts.js.map
