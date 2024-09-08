import { useState, useEffect, React} from 'react'
import axios from 'axios'


export default function UsersPage() {
    const [users, setUsers] = useState([])

    const getUsers = () => {
        axios.get('http://localhost:8080/users')
        .then(res => setUsers(res.data))
        .catch(err => console.log(err))
    }

    useEffect(() => {
        getUsers()
    }, [])

  return (
    <div className='bg-slate-100'>
        <div className='text-orange-500 text-2xl'>Users pages</div>
        <button className=' rounded-lg bg-orange-100 m-12 p-4'>add user</button>
        <div className='rounded-lg bg-emerald-100 m-12 p-4'>
            
            <hr className="h-px my-8 bg-gray-200 border-0 dark:bg-gray-700"></hr>
            <div className='grid grid-cols-4  my-1 bg-emerald-100 flex flex-center'> <div>id</div> <div>name</div> <div>update</div> <div>delete</div></div>
            <hr className="h-px my-8 bg-gray-200 border-0 dark:bg-gray-700"></hr>
            {users.map(user => <div><div key={user.id} className='grid grid-cols-4  my-1 bg-emerald-100 flex flex-center'> <div> {user.id}</div> <div> {user.name}</div> <div><button className='bg-cyan-400 rounded-lg px-2'>update</button></div> <div><button className='bg-cyan-400 rounded-lg px-2'>delete</button></div></div> <hr className="h-px my-3 bg-gray-200 border-0 dark:bg-gray-700"></hr> </div>)}
        </div>
    </div>
  )
}
