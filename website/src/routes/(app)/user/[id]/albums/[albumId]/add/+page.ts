import { getUserPosts } from '$lib/app/post';
import { getUserRecipes } from '$lib/app/recipe';
import { redirect } from '@sveltejs/kit';

export const load = async ({ fetch, params, parent }) => {

    const { id, album } = await parent();

    if (album.user_id != id) {
        throw redirect(301, `/user/${album.user_id}/albums/${album.id}`);
    }

    const [
        posts,
        recipes
    ] = await Promise.all([
        getUserPosts(params.id).json(fetch),
        getUserRecipes(params.id).json(fetch)
    ]);

    return {
        posts,
        recipes
    }

}
