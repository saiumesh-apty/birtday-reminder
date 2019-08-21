const AUTH_KEY = 'BIRTHDAY_LOGGEDIN';
const AUTH_KEY_USER = 'AUTH_KEY_USER';

export const isAuth = (): boolean => {
    return window.localStorage.getItem(AUTH_KEY) === "true"
}

export const setAuth = (): void => {
    window.localStorage.setItem(AUTH_KEY, String(true));
}

export const set_user = (user_id: number) => {
    window.localStorage.setItem(AUTH_KEY, String(true));
    window.localStorage.setItem(AUTH_KEY_USER, String(user_id));
}

export const get_user_id = (): number => {
    return Number(window.localStorage.getItem(AUTH_KEY_USER))
}

export const removeAuth = () => {
    window.localStorage.setItem(AUTH_KEY, String(false));
    window.localStorage.removeItem(AUTH_KEY_USER);
}