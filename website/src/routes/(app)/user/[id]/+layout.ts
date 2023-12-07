import { get } from '$lib/apiFetch';
import type { User } from '$lib/app/user';
import { redirect } from '@sveltejs/kit';

export const load = async ({ params, fetch, route }) => {

    if (route.id == '/(app)/user/[id]') {
        throw redirect(301, `/user/${params.id}/posts`);
    }
    
    const [userResponse, followersResponse, followingResponse] = await Promise.all([
        get(`user/${params.id}`).run(fetch),
        get(`user/${params.id}/followers`).run(fetch),
        get(`user/${params.id}/following`).run(fetch)
    ]);

    const [user, followers, following]: [User, User[], User[]] = await Promise.all([
        userResponse.json(),
        followersResponse.json(),
        followingResponse.json()
    ])

    return {
        user,
        followers,
        following,
        title: user.display_name
    }
}