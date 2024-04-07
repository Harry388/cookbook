<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipes from '$lib/components/recipe/recipes.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';

    export let data;

</script>

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-4">

    <div class="basis-1/3">
        <h3 class="font-bold text-lg pb-5">Post</h3>
        <Post post={data.post} />
    </div>

    {#if data.post.links > 0}
        <div class="basis-1/3">
            <h3 class="font-bold text-lg pb-5">Recipes</h3>
            <Recipes recipes={data.recipes} />
        </div>
    {/if}

    <div class="basis-1/3">
        <h3 class="font-bold text-lg pb-5">Comments</h3>
        <CommentBlock comments={data.comments} type="POST" id={data.post.id} />
    </div>

</div>

