import { redirect } from "@sveltejs/kit"

export const load = async ({ route }) => {

    if (route.id == '/(app)') {
        throw redirect(301, '/home/feed');
    }

}
