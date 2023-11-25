import { test } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url }) => {
    if (url.pathname == '/login') {
        return;
    }
    const response = await test();
    if (!response.ok) {
        throw redirect(301, `/login?redirect=${url.pathname}`);
    }
}

export const ssr = false;