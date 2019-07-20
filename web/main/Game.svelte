<script context="module">
    export const nobody = Symbol('nobody');
</script>

<script>
    import { readable, writable } from 'svelte/store';
    import { createEventDispatcher } from 'svelte';

    import Board from './Board.svelte';
    
    export let game;

	const dispatch = createEventDispatcher();

    const thinking = writable(false);
    const highlight = writable([]);

    const board = readable(game.board, (set) => game.addEventListener('boardchanged', (event) => set(event.board)));
    const moves = readable(game.validMoves, (set) => game.addEventListener('moveschanged', (event) => set(event.moves)));
    
    game.addEventListener('gameover', (event) => {
        highlight.set(event.cells || []);
        dispatch('gameover', event.winner || nobody);
    });

    async function drop(col) {
        const cell = await game.drop(col);
        if (game.over) {
            return false;
        }

        highlight.set([cell]);
        return true;
    }

    async function columnClickHandler(event) {
        if (!await drop(event.detail)) {
            return;
        }
        thinking.set(true);

        const col = await game.bestMove(1000);
        try {
            if (!await drop(col)) {
                return;
            }
        } finally {
            thinking.set(false);
        }
    }
</script>

{#if $thinking}
    <Board
        cols={game.cols}
        rows={game.rows}
        board={$board}
        moves={$moves}
        disabled={$thinking}
        highlight={$highlight}
        on:clickcolumn={columnClickHandler}
    />
    Thinking...
{:else}
    <Board
        cols={game.cols}
        rows={game.rows}
        board={$board}
        moves={$moves}
        disabled={$thinking}
        highlight={$highlight}
        on:clickcolumn={columnClickHandler}
    />
{/if}
