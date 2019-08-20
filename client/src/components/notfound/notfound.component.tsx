import React from 'react';
import { Link } from 'react-router-dom';

function NotFound() {
    return (
        <Link to="/login">Nothing found here</Link>
    )
}

export default NotFound;