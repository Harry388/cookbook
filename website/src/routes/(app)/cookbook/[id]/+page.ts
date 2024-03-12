import { getCookbook, getCookbookRecipes } from '$lib/app/cookbook';
import { test } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

export const load = async ({ fetch, params, url }) => {

    const response = await test().run(fetch);

    if (!response.ok) {
        throw redirect(301, `/login?redirect=${url.pathname}`);
    }

    const id: number = await response.json();

    const [
        cookbook,
        recipes
    ] = await Promise.all([
        getCookbook(params.id).json(fetch),
        getCookbookRecipes(params.id).json(fetch)
    ]);

    let page = Number(url.searchParams.get('p'));

    if (isNaN(page) || (page < 0)) {
        page = 0;
    }
    else if (page >= recipes.length) {
        page = recipes.length - 1;
    }

    return {
        cookbook,
        recipes,
        page,
        id
    }

}

export const ssr = false;
