import { Link, useLocation } from 'react-router-dom'

function Navbar({ user, onLogout }) {
  const location = useLocation()
  
  const navItems = [
    { path: '/dashboard', label: 'Dashboard', icon: '📊' },
    { path: '/repos', label: 'Repositories', icon: '📁' },
    { path: '/reviews', label: 'Reviews', icon: '🔍' },
    { path: '/settings', label: 'Settings', icon: '⚙️' },
  ]
  
  return (
    <nav className="bg-white border-b border-gray-200">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="flex justify-between h-16">
          <div className="flex items-center">
            <Link to="/dashboard" className="flex items-center gap-2">
              <span className="text-2xl">🦸</span>
              <span className="text-xl font-bold text-sheriff-700">CodeSheriff</span>
            </Link>
            <div className="hidden sm:flex ml-10 gap-1">
              {navItems.map(item => (
                <Link
                  key={item.path}
                  to={item.path}
                  className={`px-3 py-2 rounded-lg text-sm font-medium transition-colors ${
                    location.pathname === item.path
                      ? 'bg-sheriff-100 text-sheriff-700'
                      : 'text-gray-600 hover:bg-gray-100'
                  }`}
                >
                  <span className="mr-1">{item.icon}</span>
                  {item.label}
                </Link>
              ))}
            </div>
          </div>
          <div className="flex items-center gap-4">
            <span className="text-sm text-gray-600">@{user?.username}</span>
            <button
              onClick={onLogout}
              className="text-sm text-gray-500 hover:text-gray-700"
            >
              Logout
            </button>
          </div>
        </div>
      </div>
    </nav>
  )
}

export default Navbar
