import { getUserCookbooks } from '$lib/app/cookbook';

export const load = async ({ fetch, params }) => {

    const cookbooks = await getUserCookbooks(params.id).json(fetch);

    return {
        cookbooks
    }

}
