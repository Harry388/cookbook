<script lang="ts">

    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import { getUserRecipes } from '$lib/app/recipe';
    import { createEventDispatcher, onMount, getContext } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';

    const dispatch = createEventDispatcher();

    const id: number = getContext('id');

    export let recipes: Recipe[] = [];

    let userRecipes: Recipe[] = [];
    let currentRecipeId: number = -1;

    $: {
        recipes;
        setUserRecipes();
    }

    async function save() {
        dispatch('save', currentRecipeId);
        currentRecipeId = -1;
    }

    function cancel() {
        currentRecipeId = -1;
    }

    function show() {
        //@ts-ignore
        document.getElementById('attachRecipeModal').showModal();
    }

    onMount(setUserRecipes);

    async function setUserRecipes() {
        userRecipes = 
            (await getUserRecipes(id).json())
            .filter(r => !recipes.map(rr => rr.id).includes(r.id));
    }

</script>

<button class="btn btn-outline w-full" on:click={show}>Attach Recipe</button>
<dialog id="attachRecipeModal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg mb-5">Attach Recipe</h3>
        <div class="collapse bg-base-200">
            <input type="checkbox" />
            <div class="collapse-title text-xl font-medium">My Recipes</div>
            <div class="collapse-content">
                <div class="flex gap-5 flex-col">
                    {#each userRecipes as recipe}
                        <button class={`text-left ${(recipe.id == currentRecipeId) ? 'outline outline-primary rounded-2xl' : ''}`} on:click={() => currentRecipeId = recipe.id} > 
                            <RecipeComponent {recipe} link />
                        </button>
                    {/each}
                </div>
            </div>
        </div>
        <div class="modal-action">
            <form method="dialog">
                <button class="btn btn-ghost mr-5" on:click={cancel}>Cancel</button>
                <button class="btn" on:click={save}>Save</button>
            </form>
        </div>
    </div>
    <form method="dialog" class="modal-backdrop">
        <button on:click={cancel}>close</button>
    </form>
</dialog>
