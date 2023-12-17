<script lang="ts">

    import Image from '$lib/components/util/image.svelte';

    export let media: number[];

    let current = 0;

</script>

{#each media as id}
    <div class={media[current] == id ? '' : 'hidden'}>
        <Image src="post/media/{id}" alt="Post Image" />
    </div>
{/each}

<div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
    {#if current > 0}
        <button class="btn btn-circle" on:click={() => current--}>❮</button> 
    {:else}
        <div></div>
    {/if}
    {#if current < (media.length - 1)}
        <button class="btn btn-circle"  on:click={() => current++}>❯</button>
    {/if}
</div>