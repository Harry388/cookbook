<script lang="ts">

    import Post from '$lib/components/post/post.svelte';
    import Recipes from '$lib/components/recipe/recipes.svelte';
    import CommentBlock from '$lib/components/comment/commentBlock.svelte';

    export let data;

</script>

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

