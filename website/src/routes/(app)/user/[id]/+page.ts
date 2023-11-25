import { get } from '$lib/apiFetch';

export const load = async ({ params }) => {
    const response = await get(`user/${params.id}`);
    const user = await response.json();
    return {
        user
    }
}