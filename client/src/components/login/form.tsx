import React, { useState, FormEvent } from 'react';
import { RouteComponentProps } from 'react-router-dom';
import './login.scss';

function LoginForm(props: RouteComponentProps) {
    const [email, onEmailChange] = useState('');
    const [password, onPasswordChange] = useState('');
    const onSubmit = (e: FormEvent) => {
        e.preventDefault();
    }
    return (
        <div className="login__background">
            <form onSubmit={onSubmit}>
                <input
                    autoComplete="on"
                    className='loginform__input'
                    onChange={(e) => onEmailChange(e.target.value)}
                    value={email}
                    type="email"
                    placeholder="email" />
                <input
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