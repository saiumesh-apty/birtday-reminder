const AUTH_KEY = 'BIRTHDAY_LOGGEDIN';

export const isAuth = (): boolean => {
    return Boolean(window.localStorage.getItem(AUTH_KEY))
}

export const setAuth = (): void => {
    window.localStorage.setItem(AUTH_KEY, String(true));
}