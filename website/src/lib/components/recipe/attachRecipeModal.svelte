<script lang="ts">

    import SearchList from '$lib/components/util/searchList.svelte';
    import SearchItem from '$lib/components/util/searchItem.svelte';
    import RecipeComponent from '$lib/components/recipe/recipe.svelte';
    import { getUserRecipes } from '$lib/app/recipe';
    import { getUserAblums, getAlbumEntries } from '$lib/app/album';
    import { createEventDispatcher, onMount, getContext } from 'svelte';
    import type { Recipe } from '$lib/app/recipe';
    import type { Album } from '$lib/app/album';

    const dispatch = createEventDispatcher();

    const id: number = getContext('id');

    export let modalId = '';
    export let recipes: Recipe[] = [];

    let userRecipes: Recipe[] = [];
    let currentRecipeId: number = -1;
    let userAlbums: Album[] = [];
    let albumEntries: { [key: number]: Recipe[] } = {};

    $: {
        recipes;
        setUserRecipes();
        setUserAlbums();
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
        document.getElementById(`${modalId}attachRecipeModal`).showModal();
    }

    onMount(() => {
        setUserRecipes();
        setUserAlbums();
    });

    async function setUserRecipes() {
        userRecipes = 
            (await getUserRecipes(id).json())
            .filter(r => !recipes.map(rr => rr.id).includes(r.id));
    }

    async function setUserAlbums() {
        const albums = await getUserAblums(id).json();
        for (const album of albums) {
            const entries = await getAlbumEntries(album.id).json();
            const albumRecipes: Recipe[] = [];
            for (const entry of entries) {
                if ((entry.type == 'Recipe') && !recipes.map(r => r.id).includes(entry.id)) {
                    albumRecipes.push(entry);
                }
            }
            albumEntries[album.id] = albumRecipes;
        }
        userAlbums = albums.filter(a => albumEntries[a.id].length > 0);
    }

</script>

<button class="btn btn-outline w-full" on:click={show}>Attach Recipe</button>
<dialog id="{modalId}attachRecipeModal" class="modal modal-bottom sm:modal-middle">
    <div class="modal-box">
        <h3 class="font-bold text-lg mb-5">Attach Recipe</h3>
        <div class="collapse collapse-arrow bg-base-200">
            <input type="checkbox" />
            <div class="collapse-title text-xl font-medium">My Recipes</div>
            <div class="collapse-content">
                <div class="flex gap-5 flex-col">
                    <SearchList>
                    {#each userRecipes as recipe (recipe.id)}
                        <SearchItem key={recipe.title}>
                        <button class={`text-left ${(recipe.id == currentRecipeId) ? 'outline outline-primary rounded-2xl' : ''}`} on:click={() => currentRecipeId = recipe.id} > 
                            <RecipeComponent {recipe} link />
                        </button>
                        </SearchItem>
                    {/each}
                    </SearchList>
                </div>
            </div>
        </div>
        {#each userAlbums as album (album.id)}
            <div class="collapse collapse-arrow bg-base-200 mt-5">
                <input type="checkbox" />
                <div class="collapse-title text-xl font-medium"><span class="font-bold">Album: </span>{ album.title }</div>
                <div class="collapse-content">
                    <div class="flex gap-5 flex-col">
                        <SearchList>
                        {#each albumEntries[album.id] as recipe (recipe.id)}
                            <SearchItem key={recipe.title}>
                            <button class={`text-left ${(recipe.id == currentRecipeId) ? 'outline outline-primary rounded-2xl' : ''}`} on:click={() => currentRecipeId = recipe.id} > 
                                <RecipeComponent {recipe} link />
                            </button>
                            </SearchItem>
                        {/each}
                        </SearchList>
                    </div>
                </div>
            </div>
        {/each}
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
