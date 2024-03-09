import { getUserCookbooks } from '$lib/app/cookbook';

export const load = async ({ fetch, params, depends }) => {

    const cookbooks = await getUserCookbooks(params.id).json(fetch);

    depends('app:cookbooks');

    return {
        cookbooks
    }

}
