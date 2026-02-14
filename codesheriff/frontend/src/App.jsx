import { useState, useEffect } from 'react'
import { BrowserRouter, Routes, Route, Navigate } from 'react-router-dom'
import Login from './components/Login'
import Dashboard from './components/Dashboard'
import Repos from './components/Repos'
import Reviews from './components/Reviews'
import Settings from './components/Settings'
import Navbar from './components/Navbar'

function App() {
  const [user, setUser] = useState(null)
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    checkAuth()
  }, [])

  const checkAuth = async () => {
    try {
      const res = await fetch('/api/auth/status')
      const data = await res.json()
      if (data.authenticated) {
        setUser({ username: data.username })
      }
    } catch (e) {
      console.error('Auth check failed:', e)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="animate-spin rounded-full h-12 w-12 border-b-2 border-sheriff-600"></div>
      </div>
    )
  }

  return (
    <BrowserRouter>
      <div className="min-h-screen bg-gray-50">
        {user && <Navbar user={user} onLogout={() => setUser(null)} />}
        <Routes>
          <Route path="/login" element={!user ? <Login /> : <Navigate to="/dashboard" />} />
          <Route path="/dashboard" element={user ? <Dashboard /> : <Navigate to="/login" />} />
          <Route path="/repos" element={user ? <Repos /> : <Navigate to="/login" />} />
          <Route path="/reviews" element={user ? <Reviews /> : <Navigate to="/login" />} />
          <Route path="/settings" element={user ? <Settings /> : <Navigate to="/login" />} />
          <Route path="/" element={<Navigate to={user ? "/dashboard" : "/login"} />} />
        </Routes>
      </div>
    </BrowserRouter>
  )
}

export default App
