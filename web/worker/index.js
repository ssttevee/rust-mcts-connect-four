import init, { Game, MCTS } from '../../pkg/libc4';

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
}

init('mcts_bg.wasm').then(() => (self.mcts = new MCTS(), postMessage({ ready: true })), (error) => postMessage({ error }))
