<script lang="ts">

    import { PUBLIC_API_URL } from "$env/static/public";
    import Image from '$lib/components/util/image.svelte';

    export let media: { [key: number]: string };

    $: ids = Object.keys(media);

    let current = 0;

</script>

{#each Object.entries(media) as [id, type]}
    <div class={ids[current] == id ? 'm-auto' : 'hidden'}>
        {#if type == 'image'}
            <Image src="post/media/{id}" alt="Post Image" />
        {:else if type == 'video'}
            <video controls>
                <track kind="captions">
                <source src="{PUBLIC_API_URL}/post/media/{id}">
            </video>
        {/if}
    </div>
{/each}

<div class="absolute flex justify-between transform -translate-y-1/2 left-5 right-5 top-1/2">
    {#if current > 0}
        <button class="btn btn-circle" on:click={() => current--}>❮</button> 
    {:else}
        <div></div>
    {/if}
    {#if current < (ids.length - 1)}
        <button class="btn btn-circle"  on:click={() => current++}>❯</button>
    {/if}
</div>
