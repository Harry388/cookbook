import { redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {

    const { ownsRecipe, recipe } = await parent();

    if (!ownsRecipe) {
        throw redirect(301, `/recipe/${recipe.id}`);
    }

    return;

}