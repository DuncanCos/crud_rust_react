import React from 'react'

import { Link } from "react-router-dom";

export default function Navbar() {
  return (
    <div>
        <Link to="/">test</Link>
        <Link to="/user/:iduser">user</Link>
        <Link to="/users">users</Link>
    </div>
  )
}
