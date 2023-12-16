import { getPost } from '$lib/app/post';

export const load = async ({ params, fetch }) => {

    const post = await getPost(params.id, fetch);

    return {
        post,
        title: post.title
    }

}