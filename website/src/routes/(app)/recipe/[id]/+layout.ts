import { getRecipe, getRecipePosts } from '$lib/app/recipe';

export const load = async ({ params, fetch, parent }) => {

    const { id } = await parent();
    
    const [recipe, posts] = await Promise.all([getRecipe(params.id, fetch), getRecipePosts(params.id, fetch)]);

    return {
        recipe,
        posts,
        title: recipe.title,
        ownsRecipe: id == recipe.user_id
    }
}
