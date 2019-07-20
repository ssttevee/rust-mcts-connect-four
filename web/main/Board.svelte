<script>
    import { createEventDispatcher, onDestroy } from 'svelte';
    import { player1, player2 } from './game';

	const dispatch = createEventDispatcher();
	function clickColumn(col) {
		dispatch('clickcolumn', col);
	}

    export let cols;
    export let rows;
    export let board;
    export let moves;
    export let disabled;
    export let highlight;

    let hovered_column = null;
    // if (hovered_column !== null && !valid_moves.includes(hovered_column)) {
    //     hovered_column = null;
    // }

    function isValidColumn(col) {
        return !disabled && moves.includes(col);
    }

    function isHighlighted(col, row) {
        return highlight.some(([x, y]) => x == col && row == y);
    }

    function cellClass(col, row) {
        switch (board[col][row]) {
            case player1:
                return 'player-1';
            case player2:
                return 'player-2';
        }

        return 'empty';
    }
</script>

<style>
td {
    width: 50px;
    height: 50px;
    border: 1px gray solid;
    text-align: center;
}
td:not(.empty)::before {
    content: '\25CF';
}
td.player-1 {
    color: red;
}
td.player-2 {
    color: green;
}
td.hovered:not(.highlighted) {
    background-color: aquamarine;
}
td.highlighted {
    background-color: gold;
}
</style>

<table on:mouseout={() => hovered_column = null}>
    {#each [...Array(rows).keys()].reverse() as row}
        <tr>
            {#each {length: cols} as _, col}
                <td
                    on:click={() => isValidColumn(col) && clickColumn(col)}
                    on:mouseover={() => hovered_column = isValidColumn(col) ? col : null}
                    class="{hovered_column == col ? 'hovered' : ''} {isHighlighted(col, row) ? 'highlighted' : ''} {cellClass(col, row)}"
                ></td>
            {/each}
        </tr>
    {/each}
</table>
