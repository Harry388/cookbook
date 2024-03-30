import { getCookbook, getCookbookPages } from '$lib/app/cookbook';

export const load = async ({ fetch, params, depends }) => {

    const [
        cookbook,
        pages
    ] = await Promise.all([
        getCookbook(params.id).json(fetch),
        getCookbookPages(params.id).json(fetch)
    ]);

    depends('app:cookbook');

    return {
        cookbook,
        pages,
        title: cookbook.title
    }

}
