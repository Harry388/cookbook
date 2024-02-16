import { redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {

    const { id, user } = await parent();

    if (user.id != id) {
        throw redirect(301, `/user/${user.id}/albums`);
    }

}
