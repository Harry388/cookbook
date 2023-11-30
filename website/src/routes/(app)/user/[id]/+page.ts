import { get } from '$lib/apiFetch';
import type { User } from '$lib/app/user';

export const load = async ({ params, fetch }) => {
    const response = await get(`user/${params.id}`).run(fetch);
    const user: User = await response.json();
    return {
        user,
        title: user.display_name
    }
}