const init = (() => {
    function randomString() {
        return Math.random().toString().slice(2);
    }

    function wrapWorker(w) {
        let resolvers = {};
        w.addEventListener('message', (event) => {
            const { id, result, error } = event.data;
            if (!(id in resolvers)) {
                return;
            }
            
            const { resolve, reject } = resolvers[id];
            delete resolvers[id];

            if (error) {
                reject(error);
            } else {
                resolve(result);
            }
        });

        w.addEventListener('error', (event) => {
            const local = resolvers;
            resolvers = {};
            Object.values(local).map(({ reject }) => reject(event));
        });

        return {
            send(name, payload) {
                let id;
                while ((id = randomString()) && id in resolvers) {}

                return new Promise((resolve, reject) => {
                    resolvers[id] = { resolve, reject };
                    w.postMessage({ id, name, payload });
                });
            }
        }
    }

    let worker;
    return async function() {
        if (worker) {
            return worker;
        }
        
        return worker = new Promise((resolve, reject) => {
            const w = new Worker('mcts.js');
            const onerr = () => {
                worker = null;
                reject(new Error('failed to initialize worker'));
            };
            const onmsg = (event) => {
                w.removeEventListener('error', onerr);
                if (event.data.error) {
                    worker = null;
                    reject(event.data.error);
                } else {
                    resolve(worker = wrapWorker(w));
                }
            };
            w.addEventListener('message', onmsg);
            w.addEventListener('error', onerr);
        });
    }
})();

export default init;

export const player1 = Symbol('Player 1');
export const player2 = Symbol('Player 2');

function convertBoard(board) {
    return board.map((col) => col.map((token) => token === 0 ? player1 : token === 1 ? player2 : null))
}

class MovesChangedEvent extends Event {
    constructor(moves) {
        super('moveschanged');
        this.moves = moves;
    }
}

class BoardChangedEvent extends Event {
    constructor(board) {
        super('boardchanged');
        this.board = board;
    }
}

class GameOverEvent extends Event {
    constructor(winner, cells) {
        super('gameover');
        this.winner = winner;
        this.cells = cells;
    }
}

export class Game extends EventTarget {
    static async create(opts = {}) {
        const worker = await init();
        return new Game(worker, await worker.send('newGame', opts));
    }

    constructor(worker, { id, cols, rows, moves, board }) {
        super();
        this.__worker = worker;
        this.__id = id;
        this.__cols = cols;
        this.__rows = rows;
        this.__moves = moves;
        this.__board = convertBoard(board);
        this.__lastMove = null;
        this.__over = false;
    }

    get validMoves() {
        return this.__moves;
    }

    get board() {
        return this.__board;
    }

    get cols() {
        return this.__cols;
    }

    get rows() {
        return this.__rows;
    }

    get over() {
        return this.__over;
    }

    async drop(column) {
        if (this.__over) {
            throw new Error('game is already over');
        }

        const { cell, over, cells, moves, board } = await this.__worker.send('drop', { gameId: this.__id, column });

        this.dispatchEvent(new BoardChangedEvent(this.__board = convertBoard(board)));

        if (this.__over = over) {
            this.dispatchEvent(new MovesChangedEvent(this.__moves = []));
            this.dispatchEvent(new GameOverEvent(cells ? this.__board[cell[0]][cell[1]] : null, cells));
            await this.__worker.send('freeGame', { gameId: this.__id });
        } else if (this.__moves.length !== moves.length) {
            this.dispatchEvent(new MovesChangedEvent(this.__moves = moves));
        }

        return cell;
    }

    async bestMove(thinkingTime) {
        if (this.__over) {
            throw new Error('game is already over');
        }

        const moves = await this.__worker.send('think', { gameId: this.__id, duration: thinkingTime });
        return moves.reduce((best, [move, weight]) => !best || best[1] < weight ? [move, weight] : best, null)[0];
    }
}
