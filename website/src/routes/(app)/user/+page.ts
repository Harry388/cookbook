import { redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {

    const { id } = await parent();

    throw redirect(301, `/user/${id}/posts`);
    
}