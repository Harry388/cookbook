<script lang="ts">

    import PostPreview from '$lib/components/post/postPreview.svelte';
    import Recipes from '$lib/components/recipe/recipes.svelte';
    import { deletePost } from '$lib/app/post';
    import { goto } from '$app/navigation';
    import { writable } from 'svelte/store';
    import { setContext } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';

    export let data;

    const recipes = writable<Recipe[]>();
    $: recipes.set(data.recipes);
    setContext('recipes', recipes);

    async function onDelete() {
        if (!confirm('Are you sure?')) return;
        const response = await deletePost(data.post.id);
        if (response.ok) {
            goto('/user');
        }
    }

</script>

{#if data.ownsPost}
    <a class="btn btn-primary" href="/post/{data.post.id}/edit">Edit Post</a>
    <button class="btn btn-error" on:click={onDelete}>Delete Post</button>
{/if}

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-x-72 gap-y-5">

    <PostPreview post={data.post} />
    
    <div>

        <Recipes />

    </div>

</div>

