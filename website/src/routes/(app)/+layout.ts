import { test } from '$lib/auth/auth';
import { redirect } from '@sveltejs/kit';

export const load = async ({ url }) => {
    const response = await test();
    if (!response.ok) {
        throw redirect(301, `/login?redirect=${url.pathname}`);
    }
    const id: number = await response.json();
    return {
        id
    }
}

export const ssr = false;