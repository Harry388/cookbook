<script lang="ts">

    import type { Book } from '$lib/app/page';

    export let book: Book;

</script>

<div class="pt-20 bg-base-200 m-auto w-fit">
    <h2 class="text-5xl font-bold">Contents</h2>
    {#each book as section}
        <h2 class="text-4xl font-bold py-5">{ section.section.position + 1}. { section.section.title }</h2>
        {#each section.recipes as recipe}
            <h3 class="text-2xl">{ recipe.title }</h3>
        {/each}
    {/each}
</div>
