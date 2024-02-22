import { getCommunity, getCommunityPosts } from '$lib/app/community';
import type { Post } from '$lib/app/post';

export const load = async ({ fetch, params, depends }) => {

    const community = await getCommunity(params.id).json(fetch);

    let posts: Post[] = [];

    if (community.public || community.is_member) {
        posts = await getCommunityPosts(params.id).json(fetch);
    }

    depends('app:community');

    return {
        community,
        posts,
        title: community.title
    }

}
