import { test } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url, fetch, route }) => {

    const response = await test().run(fetch);

    if (!response.ok) {
        throw redirect(301, `/login?redirect=${url.pathname}`);
    }

    const id: number = await response.json();

    return {
        id
    }
}

export const ssr = false
