import { get } from '$lib/apiFetch';

export const load = async ({ params, fetch }) => {
    const response = await get(`user/${params.id}`).run(fetch);
    const user = await response.json();
    return {
        user
    }
}