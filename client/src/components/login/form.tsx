import React, { useState, FormEvent } from 'react';
import { RouteComponentProps } from 'react-router-dom';
import './login.scss';
import { loginAPI, extractError } from '../../http';
import { set_user } from '../../utils/auth.util';

function LoginForm(props: RouteComponentProps) {
    const [email, onEmailChange] = useState('');
    const [password, onPasswordChange] = useState('');
    const onSubmit = async (e: FormEvent) => {
        e.preventDefault();
        try {
            const res = await loginAPI({email, password})
            set_user(res.user_id);
            props.history.push('/dashboard');
        } catch(error) {
            alert(extractError(error));
        }
    }
    return (
        <div className="login__background">
            <form onSubmit={onSubmit}>
                <label htmlFor="email">Email</label>
                <input
                    name="email"
                    autoComplete="on"
                    className='loginform__input'
                    onChange={(e) => onEmailChange(e.target.value)}
                    value={email}
                    type="email"
                    placeholder="email" />
                <label htmlFor="password">Password</label>
                <input
                    name="password"
                    autoComplete="on"
                    className='loginform__input'
                    onChange={(e) => onPasswordChange(e.target.value)}
                    value={password}
                    type="password"
                    placeholder="password" />
                <button type='submit'>Login</button>
            </form>
        </div>
    )
}

export default LoginForm;