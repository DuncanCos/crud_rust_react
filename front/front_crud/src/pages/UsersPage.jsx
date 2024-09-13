import { useState, useEffect, React} from 'react'
import axios from 'axios'
import { useNavigate } from 'react-router-dom';


export default function UsersPage() {
    const [users, setUsers] = useState([])
    let navigate = useNavigate();

    const getUsers = () => {
        axios.get('http://localhost:8080/users/users')
        .then(res => {
            setUsers(res.data)
            console.log(res.data)
        })
        .catch(err => console.log(err))
    }

    const redirectNewUser = () => {
       
        console.log('redirect')
        navigate("/newUser");
    }

    const deleteUser = (id) => {
        axios.delete(`http://localhost:8080/users/user/${id}`)
        .then(res => {
            console.log(res.data)
            getUsers()
        })
        .catch(err => console.log(err))
    }

    const updateUser = (id) => {
        navigate(`/user/${id}`)
    }

    useEffect(() => {
        getUsers()
    }, [])

  return (
    <div className='bg-slate-100'>
        <div className='text-orange-500 text-2xl'>Users pages</div>
        <button className=' rounded-lg bg-orange-100 m-12 p-4' onClick={redirectNewUser} >add user</button>
        <div className='rounded-lg bg-emerald-100 m-12 p-4'>
            
            <hr className="h-px my-8 bg-gray-200 border-0 dark:bg-gray-700"></hr>
            <div className='grid grid-cols-4  my-1 bg-emerald-100 flex flex-center text-xl'> <div>Id</div> <div>Name</div> <div>Update</div> <div>Delete</div></div>
            <hr className="h-px my-8 bg-gray-200 border-0 dark:bg-gray-700"></hr>
            {users.map(user => <div><div key={user.id} className='grid grid-cols-4  my-1 bg-emerald-100 flex flex-center'> <div> {user.id}</div> <div> {user.name}</div> <div><button onClick={() => updateUser(user.id)} className='bg-cyan-400 rounded-lg px-2'>update</button></div> <div><button onClick={() => deleteUser(user.id)} className='bg-cyan-400 rounded-lg px-2'>delete</button></div></div> <hr className="h-px my-3 bg-gray-200 border-0 dark:bg-gray-700"></hr> </div>)}
        </div>
    </div>
  )
}
