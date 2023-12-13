import { get } from '$lib/apiFetch';
import type { User } from '$lib/app/user';

export const load = async ({ parent }) => {

    const { id } = await parent();
    
    const userResponse = await get(`user/${id}`).run(fetch);

    const user: User = await userResponse.json();

    return {
        user,
        title: user.display_name,
        self: id == user.id
    }
}