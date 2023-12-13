import { get } from '$lib/apiFetch';
import type { Follow } from '$lib/app/follow';

export const load = async ({ params, fetch, parent }) => {
    
    const { id } = await parent();

    const [responseFollowers, responseFollowing] = await Promise.all([
        get(`user/${params.id}/followers`).run(fetch),
        get(`user/${params.id}/following`).run(fetch)
    ]);

    const [followers, following]: [Follow[], Follow[]] = await Promise.all([
        responseFollowers.json(),
        responseFollowing.json()
    ]);

    return {
        followers,
        following,
        self: id == Number(params.id),
        userId: Number(params.id)
    }
}