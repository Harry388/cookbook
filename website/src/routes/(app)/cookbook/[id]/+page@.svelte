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
    import { formatPageArray } from '$lib/app/page.js';

    export let data;

    $: book = formatPageArray(data.pages);

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
    {#each book as section}
        <Page title={section.section.title} header>
            <Section section={section.section} />
        </Page>
        {#each section.recipes as recipe}
            <Page title={recipe.title}>
                <Recipe {recipe} />
            </Page>
            <Page>
                <Image src="cookbook/{data.cookbook.id}/section/{section.section.id}/recipe/{recipe.id}/image" width="m-auto" imageClass="shadow-lg rounded-lg" style="height: 80vh" />
            </Page>
        {/each}
    {/each}
    <Page hideNumber>
        <Index />
    </Page>
</Book>
