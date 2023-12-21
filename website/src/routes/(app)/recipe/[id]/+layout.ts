import { getRecipe } from '$lib/app/recipe';

export const load = async ({ params, fetch, parent }) => {

    const { id } = await parent();
    
    const recipe = await getRecipe(params.id, fetch);

    return {
        recipe,
        title: recipe.title,
        ownsRecipe: id == recipe.user_id
    }
}