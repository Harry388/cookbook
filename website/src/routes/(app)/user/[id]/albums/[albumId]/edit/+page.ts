import { redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {

    const { id, album } = await parent();

    if (album.user_id != id) {
        throw redirect(301, `/user/${album.user_id}/albums/${album.id}`);
    }

}
