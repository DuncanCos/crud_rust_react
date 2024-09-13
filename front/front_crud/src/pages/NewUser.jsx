import { React, useState } from 'react'
import axios from 'axios'
import { useNavigate } from 'react-router-dom';

export default function NewUser() {

    const [user, setUser] = useState({
        name: ""
    })
    let navigate = useNavigate();

    const createUser =  () => {
        axios.post('http://localhost:8080/users/user', user)
        .then(res => {
            console.log(res.data)
            navigate("/users");
        })
        .catch(err => console.log(err))
    }

  return (
    <>
        <div className='flex justify-center items-center flex-col bg-red-400 dark:bg-gray-900'>
            <div className='block mb-2 text-2xl font-medium text-black dark:text-white ' >New User :</div>
            <div><label className='block mb-2 text-sm font-medium text-black dark:text-white'> name </label><input placeholder='name' onChange={(e) => setUser({...user, name: e.target.value})} className='bg-gray-50 border border-gray-300 text-gray-900 text-sm rounded-lg focus:ring-blue-500 focus:border-blue-500 block w-full p-2.5 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500'></input></div>
            <div className='text-blue-700 hover:text-white border border-blue-700 hover:bg-blue-800 focus:ring-4 focus:outline-none focus:ring-blue-300 font-medium rounded-lg text-sm px-5 py-2.5 text-center me-2 mb-2 dark:border-blue-500 dark:text-blue-500 dark:hover:text-white dark:hover:bg-blue-500 dark:focus:ring-blue-800'><button onClick={createUser}>create user</button></div>
        </div>
    </>
  )
}
