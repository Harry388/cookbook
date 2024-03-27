import { getCookbook, getCookbookRecipes } from '$lib/app/cookbook';
import { getUserRecipes } from '$lib/app/recipe';

export const load = async ({ fetch, parent, params, depends }) => {

    const { id } = await parent();

    const [
        cookbook,
        recipes,
        userRecipes 
    ] = await Promise.all([
        getCookbook(params.id).json(fetch),
        getCookbookRecipes(params.id).json(fetch),
        getUserRecipes(id).json(fetch)
    ]);

    depends('app:cookbook');

    return {
        cookbook,
        recipes,
        userRecipes,
        title: cookbook.title
    }

}
