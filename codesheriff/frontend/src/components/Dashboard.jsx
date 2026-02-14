import { useState, useEffect } from 'react'

function Dashboard() {
  const [stats, setStats] = useState(null)
  const [repos, setRepos] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchDashboard()
  }, [])

  const fetchDashboard = async () => {
    try {
      const [statsRes, reposRes] = await Promise.all([
        fetch('/api/dashboard'),
        fetch('/api/repos')
      ])
      setStats(await statsRes.json())
      setRepos(await reposRes.json())
    } catch (e) {
      console.error('Failed to fetch dashboard:', e)
    } finally {
      setLoading(false)
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-sheriff-600"></div>
      </div>
    )
  }

  const riskColors = {
    low: 'bg-green-500',
    medium: 'bg-yellow-500',
    high: 'bg-orange-500',
    critical: 'bg-red-500',
    unknown: 'bg-gray-400'
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Dashboard</h1>
      
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-3 gap-6 mb-8">
        <div className="card">
          <div className="text-sm text-gray-500 mb-1">Total Reviews</div>
          <div className="text-3xl font-bold text-sheriff-600">{stats?.total_reviews || 0}</div>
        </div>
        <div className="card">
          <div className="text-sm text-gray-500 mb-1">This Week</div>
          <div className="text-3xl font-bold text-sheriff-600">{stats?.recent_reviews || 0}</div>
        </div>
        <div className="card">
          <div className="text-sm text-gray-500 mb-1">Monitored Repos</div>
          <div className="text-3xl font-bold text-sheriff-600">{repos.length}</div>
        </div>
      </div>
      
      {/* Risk Distribution */}
      <div className="card mb-8">
        <h2 className="text-lg font-semibold mb-4">Risk Distribution</h2>
        <div className="flex gap-4">
          {Object.entries(stats?.risk_distribution || {}).map(([level, count]) => (
            <div key={level} className="flex items-center gap-2">
              <div className={`w-3 h-3 rounded-full ${riskColors[level] || 'bg-gray-400'}`}></div>
              <span className="text-gray-600 capitalize">{level}: {count}</span>
            </div>
          ))}
          {Object.keys(stats?.risk_distribution || {}).length === 0 && (
            <p className="text-gray-500">No reviews yet</p>
          )}
        </div>
      </div>
      
      {/* Recent Activity */}
      <div className="card">
        <h2 className="text-lg font-semibold mb-4">Quick Start</h2>
        <div className="space-y-4">
          {repos.length === 0 ? (
            <div className="text-center py-8">
              <div className="text-4xl mb-3">📁</div>
              <p className="text-gray-600 mb-4">No repositories connected yet</p>
              <a href="/repos" className="btn btn-primary">Add Repository</a>
            </div>
          ) : (
            <div>
              <p className="text-gray-600 mb-4">
                You have {repos.length} repository{repos.length !== 1 ? 'ies' : ''} connected.
              </p>
              <a href="/repos" className="btn btn-primary">View PRs</a>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}

export default Dashboard
