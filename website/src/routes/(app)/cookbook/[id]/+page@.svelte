<script lang="ts">

    import Cover from '$lib/components/cookbook/cover.svelte';
    import Title from '$lib/components/cookbook/title.svelte';
    import Contents from '$lib/components/cookbook/contents.svelte';
    import Index from '$lib/components/cookbook/index.svelte';
    import Control from '$lib/components/cookbook/control.svelte';
    import Section from '$lib/components/cookbook/section.svelte';
    import Recipe from '$lib/components/cookbook/recipe.svelte';
    import Book from '$lib/components/cookbook/book.svelte';
    import Page from '$lib/components/cookbook/page.svelte';
    import Image from '$lib/components/util/image.svelte';

    export let data;

</script>

<svelte:head>
    <title>{ data.cookbook.title } - Cookbook</title>
</svelte:head>

<Book page={data.page}>

    <Control />

    <Page hideNumber>
        <Cover cookbook={data.cookbook} />
    </Page>
    <Page hideNumber>
        <Title cookbook={data.cookbook} />
    </Page>
    <Page hideNumber>
        <Contents />
    </Page>
    {#each data.pages as page}
        {#if page.type == 'Section'}
            <Page title={page.title} header>
                <Section section={page} />
            </Page>
        {:else if page.type == 'Recipe'}
            <Page title={page.title}>
                <Recipe recipe={page} />
            </Page>
        {:else if page.type == 'Image'}
            <Page>
                <Image src={page.image} imageClass="shadow-lg rounded-lg" style="height: 80vh" />
            </Page>
        {/if}
    {/each}
    <Page hideNumber>
        <Index />
    </Page>
</Book>
