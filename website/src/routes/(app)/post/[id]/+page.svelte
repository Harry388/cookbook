<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipes from '$lib/components/recipe/recipes.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';
    import { deletePost } from '$lib/app/post';
    import { addAlbumEntry } from '$lib/app/album';
    import { goto } from '$app/navigation';

    export let data;

    let newAlbum: number;

    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deletePost(data.post.id).run();
        if (response.ok) {
            goto('/user');
        }
    }

    function addAlbum() {
       addAlbumEntry(newAlbum, data.post.id, 'post').run();
    }

</script>

{#if data.ownsPost}
    <a class="btn btn-primary" href="/post/{data.post.id}/edit">Edit Post</a>
    <button class="btn btn-error" on:click={onDelete}>Delete Post</button>
{/if}

<label class="form-control w-full max-w-xs">
    <div class="label">
        <span class="label-text">Pick Album</span>
    </div>
    <select bind:value={newAlbum} class="select select-bordered">
        <option value={-1} selected>Pick one</option>
        {#each data.albums as album}
            <option value={album.id}>{ album.title }</option>
        {/each}
    </select>
    <button class="btn btn-primary w-fit my-5" on:click={addAlbum}>Save to Album</button>
</label>

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-4">

    <div class="basis-1/3">
        <Post post={data.post} />
    </div>

    <div class="basis-1/3">
        <CommentBlock comments={data.comments} type="POST" id={data.post.id} />
    </div>
    
    <div class="basis-1/3">
        <Recipes recipes={data.recipes} />
    </div>

</div>

