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

    return {
        cookbook,
        pages,
        page,
        id
    }

}

export const ssr = false;
