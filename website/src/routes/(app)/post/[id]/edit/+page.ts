import { redirect } from '@sveltejs/kit';

export const load = async ({ parent }) => {

    const { ownsPost, post } = await parent();

    if (!ownsPost) {
        throw redirect(301, `/post/${post.id}`);
    }

    return;

}