import { getRecipe, getRecipePosts } from '$lib/app/recipe';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();
    
    const [recipe, posts] = await Promise.all([getRecipe(params.id).json(fetch), getRecipePosts(params.id).json(fetch)]);

    depends('app:recipe');

    return {
        recipe,
        posts,
        title: recipe.title,
        ownsRecipe: id == recipe.user_id
    }
}
