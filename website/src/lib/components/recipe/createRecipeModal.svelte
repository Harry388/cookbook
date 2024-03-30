<script lang="ts">

    import CreateRecipe from "$lib/components/recipe/createRecipe.svelte";
    import { createRecipe } from "$lib/app/recipe";
    import { createEventDispatcher } from "svelte";
    import type { Tag } from "$lib/app/tag";

    const dispatch = createEventDispatcher();

    let title = '';
    let description = '';
    let ingredients: string[];
    let method: string[];
    let tags: Tag[];

    async function save() {
        const t = tags.map(t => t.tag);
        const response = await createRecipe(title, description, ingredients, method, t).run();
        if (response.ok) {
            title = '';
            description = '';
            method = [];
            tags = [];
            const id: number = await response.json();
            dispatch('save', id);
        }
    }

    function show() {
        //@ts-ignore
        document.getElementById('createRecipeModal').showModal();
    }

</script>

<button class="btn btn-outline" on:click={show}>Create New Recipe</button>
<dialog id="createRecipeModal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg">Create New Recipe</h3>
        <CreateRecipe bind:title={title} bind:description={description} bind:ingredients={ingredients} bind:method={method} bind:tags={tags} />
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-ghost mr-5">Cancel</button>
                <button class="btn" on:click={save}>Save</button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button>close</button>
    </form>
</dialog>
