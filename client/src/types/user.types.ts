export interface Login {
    email: string;
    password: string;
}

export interface LoginResponse {
    user_id: number;
}

export interface DOB {
    user_id: number;
    dob: String;
}

export interface UpdatePassword {
    id: number;
    password: String;
}