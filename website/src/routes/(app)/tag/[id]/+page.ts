import { getTagEntries, getTag } from '$lib/app/tag';

export const load = async ({ fetch, params, depends }) => {

    const [
        entries,
        tag
    ] = await Promise.all([
        getTagEntries(params.id).json(fetch),
        getTag(params.id).json(fetch) 
    ]);

    depends('app:tag');

    return {
        entries,
        tag,
        title: tag.tag
    }

}
