import { getPost, getPostRecipes, getPostComments } from '$lib/app/post';
import { getUserAblums } from '$lib/app/album';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();

    const [
        post, 
        recipes,
        comments,
        albums
    ] = await Promise.all([
        getPost(params.id).json(fetch),
        getPostRecipes(params.id).json(fetch),
        getPostComments(params.id).json(fetch),
        getUserAblums(id).json(fetch)
    ]);

    depends('app:post');
    
    return {
        post,
        recipes,
        comments,
        albums,
        title: post.title,
        ownsPost: id == post.user_id
    }

}
