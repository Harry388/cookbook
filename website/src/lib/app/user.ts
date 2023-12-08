export type User = {
    id: number,
    username: string,
    display_name: string,
    bio: string | null,
    pfp: string | null,
    public: number,
    following: number,
    followers: number,
    is_following: number
}