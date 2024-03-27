import { getCookbook, getCookbookPages } from '$lib/app/cookbook';
import { getUserRecipes } from '$lib/app/recipe';

export const load = async ({ fetch, parent, params, depends }) => {

    const { id } = await parent();

    const [
        cookbook,
        pages,
        userRecipes 
    ] = await Promise.all([
        getCookbook(params.id).json(fetch),
        getCookbookPages(params.id).json(fetch),
        getUserRecipes(id).json(fetch)
    ]);

    depends('app:cookbook');

    return {
        cookbook,
        pages,
        userRecipes,
        title: cookbook.title
    }

}
