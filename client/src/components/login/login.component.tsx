import React, { Fragment } from 'react';
import { Redirect } from 'react-router';
import { isAuth } from '../../utils/auth.util';
import LoginForm from './form';

function Login(props: any) {
    return (
        <Fragment>
            { isAuth() ? <Redirect to="/dashboard" /> : <LoginForm {...props}/> }
        </Fragment>

    )
}

export default Login;