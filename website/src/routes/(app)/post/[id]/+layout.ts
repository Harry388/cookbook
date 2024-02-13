import { getPost, getPostRecipes, getPostComments } from '$lib/app/post';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();

    const [
        post, 
        recipes,
        comments
    ] = await Promise.all([
        getPost(params.id).json(fetch),
        getPostRecipes(params.id).json(fetch),
        getPostComments(params.id).json(fetch)
    ]);

    depends('app:post');
    
    return {
        post,
        recipes,
        comments,
        title: post.title,
        ownsPost: id == post.user_id
    }

}
