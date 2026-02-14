import { useState, useEffect } from 'react'

function Repos() {
  const [repos, setRepos] = useState([])
  const [githubRepos, setGithubRepos] = useState([])
  const [showAddModal, setShowAddModal] = useState(false)
  const [newRepo, setNewRepo] = useState('')
  const [loading, setLoading] = useState(true)
  const [adding, setAdding] = useState(false)
  const [selectedRepo, setSelectedRepo] = useState(null)
  const [pulls, setPulls] = useState([])
  const [pullsLoading, setPullsLoading] = useState(false)
  const [reviewing, setReviewing] = useState({})

  useEffect(() => {
    fetchRepos()
  }, [])

  const fetchRepos = async () => {
    try {
      const res = await fetch('/api/repos')
      setRepos(await res.json())
    } catch (e) {
      console.error('Failed to fetch repos:', e)
    } finally {
      setLoading(false)
    }
  }

  const fetchGithubRepos = async () => {
    try {
      const res = await fetch('/api/github/repos')
      setGithubRepos(await res.json())
    } catch (e) {
      console.error('Failed to fetch GitHub repos:', e)
    }
  }

  const handleAddRepo = async (e) => {
    e.preventDefault()
    setAdding(true)
    try {
      const res = await fetch('/api/repos', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ repo_full_name: newRepo })
      })
      if (res.ok) {
        setNewRepo('')
        setShowAddModal(false)
        fetchRepos()
      } else {
        const data = await res.json()
        alert(data.detail || 'Failed to add repo')
      }
    } catch (e) {
      alert('Failed to add repository')
    } finally {
      setAdding(false)
    }
  }

  const handleDeleteRepo = async (repoId) => {
    if (!confirm('Remove this repository?')) return
    try {
      await fetch(`/api/repos/${repoId}`, { method: 'DELETE' })
      fetchRepos()
      if (selectedRepo?.id === repoId) {
        setSelectedRepo(null)
        setPulls([])
      }
    } catch (e) {
      console.error('Failed to delete repo:', e)
    }
  }

  const handleViewPRs = async (repo) => {
    setSelectedRepo(repo)
    setPullsLoading(true)
    try {
      const res = await fetch(`/api/repos/${repo.id}/pulls`)
      setPulls(await res.json())
    } catch (e) {
      console.error('Failed to fetch PRs:', e)
    } finally {
      setPullsLoading(false)
    }
  }

  const handleReviewPR = async (prNumber) => {
    if (!selectedRepo) return
    setReviewing(prev => ({ ...prev, [prNumber]: true }))
    try {
      const res = await fetch('/api/review', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          repo_id: selectedRepo.id,
          pr_number: prNumber,
          post_comment: false
        })
      })
      const data = await res.json()
      if (res.ok) {
        alert(`Review complete!\n\nRisk: ${data.analysis.risk_level}\n${data.analysis.summary}`)
      } else {
        alert(data.detail || 'Failed to review PR')
      }
    } catch (e) {
      alert('Failed to review PR')
    } finally {
      setReviewing(prev => ({ ...prev, [prNumber]: false }))
    }
  }

  const handleReviewAndComment = async (prNumber) => {
    if (!selectedRepo) return
    setReviewing(prev => ({ ...prev, [prNumber]: true }))
    try {
      const res = await fetch('/api/review', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          repo_id: selectedRepo.id,
          pr_number: prNumber,
          post_comment: true
        })
      })
      const data = await res.json()
      if (res.ok) {
        alert(`Review posted as comment!\n\nRisk: ${data.analysis.risk_level}`)
      } else {
        alert(data.detail || 'Failed to post review')
      }
    } catch (e) {
      alert('Failed to post review')
    } finally {
      setReviewing(prev => ({ ...prev, [prNumber]: false }))
    }
  }

  if (loading) {
    return (
      <div className="flex items-center justify-center h-64">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-sheriff-600"></div>
      </div>
    )
  }

  return (
    <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <div className="flex justify-between items-center mb-6">
        <h1 className="text-2xl font-bold text-gray-900">Repositories</h1>
        <button
          onClick={() => { setShowAddModal(true); fetchGithubRepos() }}
          className="btn btn-primary"
        >
          + Add Repository
        </button>
      </div>
      
      {/* Repository List */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 mb-8">
        {repos.map(repo => (
          <div key={repo.id} className="card hover:shadow-md transition-shadow">
            <div className="flex justify-between items-start">
              <div>
                <h3 className="font-semibold text-gray-900">{repo.repo_name}</h3>
                <p className="text-sm text-gray-500">{repo.repo_full_name}</p>
              </div>
              <span className={`badge ${repo.enabled ? 'badge-low' : 'badge-medium'}`}>
                {repo.enabled ? 'Active' : 'Paused'}
              </span>
            </div>
            <div className="mt-4 flex gap-2">
              <button
                onClick={() => handleViewPRs(repo)}
                className="btn btn-secondary text-sm"
              >
                View PRs
              </button>
              <button
                onClick={() => handleDeleteRepo(repo.id)}
                className="text-red-500 text-sm hover:text-red-700"
              >
                Remove
              </button>
            </div>
          </div>
        ))}
        {repos.length === 0 && (
          <div className="col-span-full text-center py-12 card">
            <div className="text-4xl mb-3">📁</div>
            <p className="text-gray-600">No repositories added yet</p>
          </div>
        )}
      </div>
      
      {/* PRs Panel */}
      {selectedRepo && (
        <div className="card">
          <div className="flex justify-between items-center mb-4">
            <h2 className="text-lg font-semibold">
              Pull Requests - {selectedRepo.repo_full_name}
            </h2>
            <button
              onClick={() => { setSelectedRepo(null); setPulls([]) }}
              className="text-gray-500 hover:text-gray-700"
            >
              Close
            </button>
          </div>
          
          {pullsLoading ? (
            <div className="text-center py-8">
              <div className="animate-spin rounded-full h-6 w-6 border-b-2 border-sheriff-600 mx-auto"></div>
            </div>
          ) : pulls.length === 0 ? (
            <p className="text-gray-500 text-center py-8">No open pull requests</p>
          ) : (
            <div className="space-y-3">
              {pulls.map(pr => (
                <div key={pr.id} className="border rounded-lg p-4 hover:bg-gray-50">
                  <div className="flex justify-between items-start">
                    <div className="flex-1">
                      <a
                        href={pr.html_url}
                        target="_blank"
                        rel="noopener noreferrer"
                        className="font-medium text-sheriff-700 hover:underline"
                      >
                        #{pr.number} {pr.title}
                      </a>
                      <p className="text-sm text-gray-500">
                        by {pr.user?.login} • {new Date(pr.created_at).toLocaleDateString()}
                      </p>
                    </div>
                    <div className="flex gap-2 ml-4">
                      <button
                        onClick={() => handleReviewPR(pr.number)}
                        disabled={reviewing[pr.number]}
                        className="btn btn-secondary text-sm"
                      >
                        {reviewing[pr.number] ? 'Reviewing...' : 'Review'}
                      </button>
                      <button
                        onClick={() => handleReviewAndComment(pr.number)}
                        disabled={reviewing[pr.number]}
                        className="btn btn-primary text-sm"
                      >
                        {reviewing[pr.number] ? 'Posting...' : 'Post Comment'}
                      </button>
                    </div>
                  </div>
                </div>
              ))}
            </div>
          )}
        </div>
      )}
      
      {/* Add Modal */}
      {showAddModal && (
        <div className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50">
          <div className="bg-white rounded-xl p-6 max-w-md w-full mx-4">
            <h2 className="text-xl font-semibold mb-4">Add Repository</h2>
            <form onSubmit={handleAddRepo}>
              <label className="block text-sm font-medium text-gray-700 mb-2">
                Repository (owner/repo)
              </label>
              <input
                type="text"
                value={newRepo}
                onChange={(e) => setNewRepo(e.target.value)}
                placeholder="e.g., facebook/react"
                className="input mb-4"
                required
              />
              <div className="flex gap-3">
                <button
                  type="button"
                  onClick={() => setShowAddModal(false)}
                  className="btn btn-secondary flex-1"
                >
                  Cancel
                </button>
                <button
                  type="submit"
                  disabled={adding}
                  className="btn btn-primary flex-1"
                >
                  {adding ? 'Adding...' : 'Add'}
                </button>
              </div>
            </form>
          </div>
        </div>
      )}
    </div>
  )
}

export default Repos
