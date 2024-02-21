import { searchRecipes } from '$lib/app/search';

export const load = async ({ parent, fetch }) => {

    const { search } = await parent();

    const recipes = await searchRecipes(search).json(fetch);

    return {
        recipes
    }

}
