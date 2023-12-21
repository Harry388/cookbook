import { getUserRecipes } from '$lib/app/recipe';

export const load = async ({ params, fetch }) => {
    
    const recipes = getUserRecipes(params.id, fetch);

    return {
        recipes
    }
}