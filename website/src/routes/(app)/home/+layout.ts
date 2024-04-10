export const load = async ({ route }) => {

    return {
        path: route.id.split('/').slice(-1)[0] // for tabs
    }

}
