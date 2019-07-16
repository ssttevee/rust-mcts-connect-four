<script>
    import { createEventDispatcher, onDestroy } from 'svelte';

	const dispatch = createEventDispatcher();
	function clickColumn(col) {
		dispatch('column', col + 1);
	}

    export let board;
    export let moves;
    export let disabled;
    
    let num_cols;
    let num_rows;
    let grid;

    onDestroy(board.subscribe((board) => {
        num_cols = board.cols();
        num_rows = board.rows();
        grid = Array.from({length: num_rows}, (_, row) => Array.from({length: num_cols}, (_, col) => [col, num_rows - row - 1, board.token_at(col, num_rows - row - 1)]))
    }));

    let valid_moves;

    let hovered_column = null;

    onDestroy(moves.subscribe((moves) => {
        valid_moves = moves.map((col) => col - 1);
        if (hovered_column !== null && !valid_moves.includes(hovered_column)) {
            hovered_column = null;
        }
    }));
</script>

<style>
td {
    width: 50px;
    height: 50px;
    border: 1px gray solid;
}
td.highlighted {
    background-color: aquamarine;
}
</style>

<table on:mouseout={() => hovered_column = null}>
{#each grid as cells}
    <tr>
    {#each cells as [col, row, token]}
        <td
            on:click={() => !$disabled && valid_moves.includes(col) && clickColumn(col)}
            on:mouseover={() => hovered_column = valid_moves.includes(col) ? col : null}
            class="{hovered_column == col ? 'highlighted' : ''}"
        >
        {#if token === undefined}
            &nbsp;
        {:else}
            {token}
        {/if}
        </td>
    {/each}
    </tr>
{/each}
</table>
