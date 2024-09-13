import React from 'react'

import { Routes, Route } from 'react-router-dom'

import Menu from '../pages/Menu'
import UserPage from '../pages/UserPage'
import UsersPage from '../pages/UsersPage'
import NewUser from '../pages/NewUser'

export default function Routing() {
  return (
    <>
    <Routes>
        <Route path="/" element={<Menu/>}></Route>
        <Route path="/users" element={<UsersPage/>}></Route>
        <Route path="/user/:iduser" element={<UserPage/>}></Route>
        <Route path="/newUser" element={<NewUser/>}></Route>
    </Routes>
    </>
  )
}
