<script lang="ts">

    import Cover from '$lib/components/cookbook/cover.svelte';
    import Title from '$lib/components/cookbook/title.svelte';
    import Contents from '$lib/components/cookbook/contents.svelte';
    import Index from '$lib/components/cookbook/index.svelte';
    import Section from '$lib/components/cookbook/section.svelte';
    import Recipe from '$lib/components/cookbook/recipe.svelte';
    import Book from '$lib/components/cookbook/book.svelte';
    import Page from '$lib/components/cookbook/page.svelte';

    export let data;

</script>

<svelte:head>
    <title>{ data.cookbook.title } - Cookbook</title>
</svelte:head>

<div class="flex print:hidden">
    {#if data.id == data.cookbook.user_id}
        <a href="/cookbook/{data.cookbook.id}/edit" class="btn btn-outline">Edit</a>
    {/if}
    <a href="?p={data.page - 1}" class="btn btn-circle">❮</a> 
    <div class="flex-grow"></div>
    <a href="?p={data.page + 1}" class="btn btn-circle">❯</a>
</div>

<Book page={data.page}>
    <Page hideNumber>
        <Cover cookbook={data.cookbook} />
    </Page>
    <Page hideNumber>
        <Title cookbook={data.cookbook} />
    </Page>
    <Page hideNumber>
        <Contents pages={data.pages} />
    </Page>
    {#each data.pages as page}
        <Page hideNumber={page.type == 'Section'}>
            {#if page.type == 'Recipe'}
                <Recipe recipe={page} />
            {:else if page.type == 'Section'}
                <Section section={page} />
            {/if}
        </Page>
    {/each}
    <Page hideNumber>
        <Index pages={data.pages} />
    </Page>
</Book>
