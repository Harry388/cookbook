import { post, get } from '$lib/apiFetch';

export function login(email: string, password: string) {
    return post('auth/login', { email, password });
}

export function logout() {
    return post('auth/logout');
}

export function create(username: string, displayName: string, email: string, password: string) {
    return post('user', { username, display_name: displayName, email, password });
}

export function test() {
    return get('auth/test');
}