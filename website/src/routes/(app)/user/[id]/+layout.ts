import { get } from '$lib/apiFetch';
import type { User } from '$lib/app/user';
import { redirect } from '@sveltejs/kit';

export const load = async ({ params, fetch, route }) => {

    if (route.id == '/(app)/user/[id]') {
        throw redirect(301, `/user/${params.id}/posts`);
    }
    
    const userResponse = await get(`user/${params.id}`).run(fetch);

    const user: User = await userResponse.json();

    return {
        user,
        title: user.display_name,
        path: route.id.split('/').slice(-1)[0]
    }
}