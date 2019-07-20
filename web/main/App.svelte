<script>
    import Game, { nobody } from './Game.svelte';
    import init, { Game as GameWorker, player1, player2 } from './game';

    let gamep;
    let winner;

    function startNewGame() {
        winner = null;
        gamep = GameWorker.create();
    }
</script>

<h1>connect four</h1>

{#await init()}
    <p>loading...</p>
{:then _}
    {#if gamep}
        {#await gamep}
            <p>starting game...</p>
        {:then game}
            <Game { game } on:gameover={(event) => winner = event.detail}/>
            {#if winner}
                { winner.toString() } wins!<br>
                <button on:click={startNewGame}>Play Again</button>
            {/if}
        {:catch err}
            <p>failed to start game</p>
            {@debug err}
        {/await}
    {:else}
        <button on:click={startNewGame}>New Game</button>
    {/if}
{:catch err}
    <p>failed to initialize</p>
    {@debug err}
{/await}
