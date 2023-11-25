import type { LayoutLoad } from './$types';
import { test } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

export const load: LayoutLoad = async ({ url }) => {
    if (url.pathname == '/login') {
        return;
    }
    const response: Response = await test();
    if (!response.ok) {
        throw redirect(301, `/login?redirect=${url.pathname}`);
    }
}

export const ssr = false;