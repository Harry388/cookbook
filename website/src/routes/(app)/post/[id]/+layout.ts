import { getPost } from '$lib/app/post';

export const load = async ({ params, fetch, parent }) => {

    const { id } = await parent();

    const post = await getPost(params.id, fetch);

    return {
        post,
        title: post.title,
        ownsPost: id == post.user_id
    }

}