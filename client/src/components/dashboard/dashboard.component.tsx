import React, { Fragment } from 'react';
import { isAuth } from '../../utils/auth.util';
import { Redirect } from 'react-router';

function Dashboard() {
    return(
      <Fragment>
          { !isAuth() ? <Redirect to='/login' /> : null }
          <p>Dashboard</p>
      </Fragment>
    )
}

export default Dashboard;