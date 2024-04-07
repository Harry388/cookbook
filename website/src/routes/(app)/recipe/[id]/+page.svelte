<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';

    export let data;

</script>

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-4">

    <div class="basis-1/3">
        <h3 class="font-bold text-lg pb-5">Recipe</h3>
        <Recipe recipe={data.recipe} />
    </div>

    {#if data.recipe.links > 0}
        <div class="basis-1/3">
            <h3 class="font-bold text-lg pb-5">Posts</h3>
            <Posts posts={data.posts} />
        </div>
    {/if}

    <div class="basis-1/3">
        <h3 class="font-bold text-lg pb-5">Comments</h3>
        <CommentBlock comments={data.comments} type="RECIPE" id={data.recipe.id} />
    </div>

</div>
