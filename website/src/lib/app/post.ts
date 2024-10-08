import { get, post, remove, put } from '$lib/apiFetch';
import type { Recipe } from '$lib/app/recipe';
import type { Comment } from '$lib/app/comment';

export type Post = {
    id: number,
    title: string,
    content: string | null,
    user_id: number,
    user_display_name: string,
    community_id: number | null,
    community_title: string | null,
    media: { [key: number]: string },
    created: string,
    is_liked: number,
    likes: number,
    comments: number,
    links: number
}

export function getPost(id: number | string) {
     return get<Post>(`post/${id}`);
}

export function getUserPosts(userId: number | string) {
    return get<Post[]>(`post/user/${userId}`);
}

export function createPost(title: string, content: string, communityId: number | null, files: File[], tags: string[], recipes: number[]) {
    const formData = new FormData();
    const postStr = JSON.stringify({ title, content, community_id: communityId, recipes });
    formData.append('post', postStr);
    if (files) {
        for (const file of files) {
            formData.append('media', file);
        }
    }
    if (tags) {
        for (const tag of tags) {
            formData.append('tags', tag);
        }
    }
    return post('post', formData, {
        headers: {
            'Content-Type':  'remove'
        }
    });
}

export function updatePost(id: number | string, title: string | null, content: string | null) {
    return put(`post/${id}`, { title, content });
}

export function deletePost(id: number | string) {
    return remove(`post/${id}`);
}

export function getPostRecipes(id: number | string) {
    return get<Recipe[]>(`post/${id}/recipe`);
}

export function addPostRecipe(id: number | string, recipeId: number | string) {
    return post(`post/${id}/addrecipe/${recipeId}`);
}

export function deletePostRecipe(id: number | string, recipeId: number | string) {
    return remove(`post/${id}/removerecipe/${recipeId}`);
}

export function getPostComments(id: number | string) {
    return get<Comment[]>(`post/${id}/comment`);
}

export function createPostComment(id: number | string, content: string, replyId: number | null) {
    return post(`post/${id}/comment`, { content, reply_id: replyId });
}

export function likePost(id: number | string) {
    return post(`post/${id}/like`);
}

export function unlikePost(id: number | string) {
    return remove(`post/${id}/like`);
}
