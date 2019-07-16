<script>
    import { writable, get } from 'svelte/store';

    import Board from './Board.svelte';
    import { Game, MCTS } from '../pkg';

    let mcts = new MCTS();
    
    let game = null;
    let winner = null;

    function newgame() {
        if (game) {
            game.free();
        }

        const g = new Game();

        const { subscribe: subscribeBoard, set: setBoard } = writable(g.board());
        const { subscribe: subscribeMoves, set: setMoves } = writable(g.valid_moves());
        const { subscribe: subscribeDisabled, set: setDisabled } = writable(false);

        function checkGameOver() {
            const prevBoard = get(game.board);
            setBoard(g.board());
            prevBoard.free();
            if (g.over()) {
                const w = winner = g.winner();
                if (w === null) {
                    winner = 'It\'s a tie!';
                } else if (w === 0) {
                    winner = 'You win!';
                } else {
                    winner = 'You lose!';
                }

                console.log(winner);

                setDisabled(true);
                setMoves([]);
                return true;
            }
        }

        winner = null;
        game = {
            board: { subscribe: subscribeBoard },
            moves: { subscribe: subscribeMoves },
            thinking: { subscribe: subscribeDisabled },
            drop: (col) => {
                console.log('dropped in column', col);
                g.drop(col);
                if (checkGameOver()) {
                    return
                }

                setDisabled(true);

                const moves = g.valid_moves();
                setMoves(moves);

                console.log(mcts.think(g, 1000));
                const weights = mcts.move_weights(g.state(), moves);
                console.log(weights);
                let best_move = moves.reduce((best, _, i) => i == best || weights[best] > weights[i] ? best : i, 0);
                g.drop(moves[best_move]);

                if (checkGameOver()) {
                    return
                }
                
                setDisabled(false);
                setMoves(g.valid_moves());
            },
            free() {
                g.free();
                get(game.board).free();
            },
        };
    }
</script>

{#if game}
    <Board
        board={game.board}
        moves={game.moves}
        disabled={game.thinking}
        on:column={(event) => game.drop(event.detail)}
    />

    {#if winner}
        {winner}
        <button on:click={() => game = null}>Back</button>
    {/if}
{/if}

{#if !game}
    <button on:click={newgame}>New Game</button>
{/if}