import { getUserAblums } from '$lib/app/album';

export const load = async ({ fetch, params, depends }) => {

    const albums = await getUserAblums(params.id).json(fetch);

    depends('app:albums');

    return {
        albums
    }

}
