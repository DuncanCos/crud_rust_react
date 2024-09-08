import React from 'react'

import { useParams } from 'react-router-dom'

export default function UserPage() {

    let { iduser } = useParams()

  return (
    <div>UserPage { iduser }</div>
  )
}
