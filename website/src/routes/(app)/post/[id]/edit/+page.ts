import { redirect } from '@sveltejs/kit';
import { getUserRecipes } from '$lib/app/recipe.js';

export const load = async ({ parent, fetch }) => {

    const { ownsPost, post, id } = await parent();

    const userRecipes = await getUserRecipes(id, fetch);

    if (!ownsPost) {
        throw redirect(301, `/post/${post.id}`);
    }

    return{
        userRecipes
    };

}