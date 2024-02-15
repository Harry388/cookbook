import { getRecipe, getRecipePosts, getRecipeComments } from '$lib/app/recipe';
import { getUserAblums } from '$lib/app/album';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();
    
    const [
        recipe,
        posts,
        comments,
        albums
    ] = await Promise.all([
        getRecipe(params.id).json(fetch),
        getRecipePosts(params.id).json(fetch),
        getRecipeComments(params.id).json(fetch),
        getUserAblums(id).json(fetch)
    ]);

    depends('app:recipe');

    return {
        recipe,
        posts,
        comments,
        albums,
        title: recipe.title,
        ownsRecipe: id == recipe.user_id
    }
}
