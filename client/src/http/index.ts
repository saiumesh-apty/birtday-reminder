import axios, { AxiosError } from 'axios';
import { Login, LoginResponse, DOB, UpdatePassword } from '../types/user.types';

const BASE_URL = process.env.NODE_ENV === 'production' ? window.location.origin : 'http://localhost:3000'


interface AxiosErrorType {
    message: string;
}

export const extractError = (err: AxiosError<AxiosErrorType>) => {
    if(err.response) {
        return err.response.data.message;
    }
    console.error(err);
    return 'unknown error';
}

const postAPI = async <T, R = {}>(input: T, url: string): Promise<R> => {
    return await axios.post(`${BASE_URL}${url}`, input).then((resp) => resp.data);
}

const putAPI = async <T, R = {}>(input: T, url: string): Promise<R> => {
    return await axios.put(`${BASE_URL}${url}`, input).then((resp) => resp.data);
}

export const loginAPI = async (input: Login): Promise<LoginResponse> => {
    return await postAPI<Login, LoginResponse>(input, '/auth/login');
}

export const insertDOBAPI = async (input: DOB) => {
    return await postAPI<DOB, {}>(input, '/admin/dob');
}

export const updateDOBAPI = async (input: DOB) => {
    return await putAPI<DOB, {}>(input, '/admin/dob_update');
}

export const updatePasswordAPI = async (input: UpdatePassword) => {
    return await postAPI<UpdatePassword, {}>(input, '/auth/update_password');
}