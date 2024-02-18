import { getPost, getPostRecipes, getPostComments } from '$lib/app/post';
import { getUserAblums } from '$lib/app/album';
import { getEntryTags } from '$lib/app/tag';

export const load = async ({ params, fetch, parent, depends }) => {

    const { id } = await parent();

    const [
        post, 
        recipes,
        comments,
        albums,
        tags
    ] = await Promise.all([
        getPost(params.id).json(fetch),
        getPostRecipes(params.id).json(fetch),
        getPostComments(params.id).json(fetch),
        getUserAblums(id).json(fetch),
        getEntryTags(params.id, 'post').json(fetch)
    ]);

    depends('app:post');
    
    return {
        post,
        recipes,
        comments,
        albums,
        tags,
        title: post.title,
        ownsPost: id == post.user_id
    }

}
