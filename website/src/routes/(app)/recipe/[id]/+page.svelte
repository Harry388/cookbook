<script lang="ts">

    import Posts from '$lib/components/post/posts.svelte';
    import Recipe from '$lib/components/recipe/recipe.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';
    import Tags from '$lib/components/tag/tags.svelte';

    export let data;

</script>

<Tags tags={data.tags} />

<div class="flex flex-col items-center lg:items-start lg:flex-row  mt-5 justify-center gap-4">

    <div class="basis-1/3">
        <Recipe recipe={data.recipe} />
    </div>

    <div class="basis-1/3">
        <CommentBlock comments={data.comments} type="RECIPE" id={data.recipe.id} />
    </div>
    
    <div class="basis-1/3">
        <Posts posts={data.posts} />
    </div>

</div>
