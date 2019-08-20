import React from 'react';
import { Router, Switch, Route, Redirect } from 'react-router-dom';
import { createHashHistory } from 'history';
import './App.css';
import Login from './components/login/login.component';
import NotFound from './components/notfound/notfound.component';
import Dashboard from './components/dashboard/dashboard.component';

const App: React.FC = () => {
  return (
    <div className="App">
      <Router history={createHashHistory()}>
        <Switch>
          <Route exact path="/" render={() => <Redirect to='/login' />} />
          <Route path="/login" component={Login} />
          <Route path="/dashboard" component={Dashboard} />
          <Route component={NotFound} />
        </Switch>
      </Router>
    </div>
  );
}

export default App;
