import React, { Fragment, useState } from 'react';
import { isAuth, get_user_id, removeAuth } from '../../utils/auth.util';
import { Redirect, RouteComponentProps } from 'react-router';
import Calendar from 'react-calendar';

import './dashboard.scss';
import { insertDOBAPI, extractError, updateDOBAPI, updatePasswordAPI } from '../../http';

function Dashboard(props: RouteComponentProps) {
  const [date, updateDate] = useState(new Date());
  const [password, updatePassword] = useState('');
  const [isPassword, updateIsPassword] = useState(false);

  function formatDate(date: Date): string {
    return `${date.getFullYear()}-${date.getMonth() + 1}-${date.getDate()}`
  }

  async function insertDOB() {
    try {
      await insertDOBAPI({ dob: formatDate(date), user_id: get_user_id() })
      alert('success');
    } catch (error) {
      alert(extractError(error));
    }
  }

  async function updateDOB() {
    try {
      await updateDOBAPI({ dob: formatDate(date), user_id: get_user_id() })
      alert('success');
    } catch (error) {
      alert(extractError(error));
    }
  }

  async function updatePasswordHTTP() {
    try {
      await updatePasswordAPI({ id: get_user_id(), password: password });
      alert('success');
      updateIsPassword(false);
      updatePassword('');
    } catch (error) {
      alert(extractError(error));
    }
  }

  async function logout() {
    removeAuth();
    props.history.push('/login');
  }

  return (
    <Fragment>
      {!isAuth() ? <Redirect to='/login' /> : null}
      <div className="dashboard__background">
        <Calendar
          onChange={(event) => updateDate(event as Date)}
          value={date}
        />
        <div className="action__buttons">
          <button onClick={insertDOB}>submit</button>
          <button onClick={updateDOB}>update</button>
          <button onClick={logout}>Logout</button>
        </div>
        <div className="action__buttons">
          <label htmlFor="isPassword">Change Password</label>
          <input
            onChange={() => updateIsPassword(!isPassword)}
            type="checkbox"
            name="isPassword"
            value="Boat"
            checked={isPassword} />
        </div>
        <div hidden={!isPassword}>
          <input
            onChange={(e) => updatePassword(e.target.value)}
            type="text"
            placeholder="newpassword"
            value={password} />
          <button onClick={updatePasswordHTTP}>Update Password</button>
        </div>
      </div>
    </Fragment>
  )
}

export default Dashboard;