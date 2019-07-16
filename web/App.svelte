<script>
    import Game from './Game.svelte';
    import init from '../pkg';
</script>

{#await init()}
    <h1>loading...</h1>
{:then _}
    <Game/>
{:catch err}
    <h1>Error: {err.message}</h1>
{/await}
