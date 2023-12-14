import { getUserFollow } from '$lib/app/follow';

export const load = async ({ params, fetch, parent }) => {
    
    const { id } = await parent();

    const { followers, following } = await getUserFollow(params.id, fetch);

    return {
        followers,
        following,
        self: id == Number(params.id),
        userId: Number(params.id)
    }
}