import { getRecipe, getRecipePosts, getRecipeComments } from '$lib/app/recipe';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();
    
    const [
        recipe,
        posts,
        comments     
    ] = await Promise.all([
        getRecipe(params.id).json(fetch),
        getRecipePosts(params.id).json(fetch),
        getRecipeComments(params.id).json(fetch)
    ]);

    depends('app:recipe');

    return {
        recipe,
        posts,
        comments,
        title: recipe.title,
        ownsRecipe: id == recipe.user_id
    }
}
