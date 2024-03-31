import { getCookbook, getCookbookPages } from '$lib/app/cookbook';
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
        pages
    ] = await Promise.all([
        getCookbook(params.id).json(fetch),
        getCookbookPages(params.id).json(fetch)
    ]);

    let page = Number(url.searchParams.get('p'));

    if (isNaN(page) || (page < 0)) {
        page = 0;
    }
    else if (page >= ((pages.length * 2) + 4)) {
        page = (pages.length * 2) + 3;
    }

    return {
        cookbook,
        pages,
        page,
        id
    }

}

export const ssr = false;
