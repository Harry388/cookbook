<script lang="ts">

    import AttachRecipe from '$lib/components/recipe/attachRecipe.svelte';
    import Confirm from '$lib/components/util/confirm.svelte';
    import { addCookbookRecipe, removeCookbookSection, removeCookbookRecipe } from '$lib/app/cookbook';
    import { createEventDispatcher } from 'svelte';
    import type { BookSection } from '$lib/app/page';

    const dispatch = createEventDispatcher();

    export let cookbookId: number;
    export let section: BookSection;

    async function removeSection() {
        const response = await removeCookbookSection(cookbookId, section.section.id).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function addRecipe(event: CustomEvent<number>) {
        const response = await addCookbookRecipe(cookbookId, section.section.id, event.detail).run();
        if (response.ok) {
            dispatch('change');
        }
    }

    async function removeRecipe(event: CustomEvent<number>) {
        const response = await removeCookbookRecipe(cookbookId, section.section.id, event.detail).run();
        if (response.ok) {
            dispatch('change');
        }
    }

</script>

<h2 class="font-bold text-2xl">{ section.section.title }</h2>
<Confirm let:show on:confirm={removeSection}>
    <button on:click={show} class="btn btn-error">Delete Section</button>
</Confirm>
<AttachRecipe recipes={section.recipes} edit on:add={addRecipe} on:remove={removeRecipe} />
