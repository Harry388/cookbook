import { redirect } from "@sveltejs/kit";

export const load = async ({ parent }) => {

    const { community } = await parent();

    if (!community.is_admin) {
        throw redirect(301, `/community/${community.id}`);
    }

}
