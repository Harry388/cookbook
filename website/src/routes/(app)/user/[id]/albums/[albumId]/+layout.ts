import { getAlbum, getAlbumEntries } from '$lib/app/album';

export const load = async ({ fetch, params, depends }) => {

    const [
        album,
        entries
    ] = await Promise.all([
        getAlbum(params.albumId).json(fetch),
        getAlbumEntries(params.albumId).json(fetch)
    ]);

    depends('app:album');

    return {
        album,
        entries,
        title: album.title
    }

}
