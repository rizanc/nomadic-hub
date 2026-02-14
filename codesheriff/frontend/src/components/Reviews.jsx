import { useState, useEffect } from 'react'

function Reviews() {
  const [reviews, setReviews] = useState([])
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    fetchReviews()
  }, [])

  const fetchReviews = async () => {
    try {
      const res = await fetch('/api/reviews')
      setReviews(await res.json())
    } catch (e) {
      console.error('Failed to fetch reviews:', e)
    } finally {
      setLoading(false)
    }
  }

  const getRiskBadge = (level) => {
    const classes = {
      low: 'badge-low',
      medium: 'badge-medium', 
      high: 'badge-high',
      critical: 'badge-critical',
      unknown: 'bg-gray-100 text-gray-600'
    }
    return `badge ${classes[level] || classes.unknown}`
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
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Review History</h1>
      
      {reviews.length === 0 ? (
        <div className="card text-center py-12">
          <div className="text-4xl mb-3">🔍</div>
          <p className="text-gray-600">No reviews yet</p>
          <p className="text-sm text-gray-500 mt-2">
            Add repositories and review PRs to see history here
          </p>
          <a href="/repos" className="btn btn-primary mt-4 inline-block">
            Get Started
          </a>
        </div>
      ) : (
        <div className="space-y-4">
          {reviews.map(review => (
            <div key={review.id} className="card">
              <div className="flex justify-between items-start mb-3">
                <div>
                  <div className="flex items-center gap-3">
                    <h3 className="font-semibold text-gray-900">
                      #{review.pr_number} {review.pr_title}
                    </h3>
                    <span className={getRiskBadge(review.risk_level)}>
                      {review.risk_level?.toUpperCase() || 'UNKNOWN'}
                    </span>
                  </div>
                  <p className="text-sm text-gray-500">
                    {review.repo_full_name} • by {review.pr_author} •{' '}
                    {review.reviewed_at ? new Date(review.reviewed_at).toLocaleString() : 'Recently'}
                  </p>
                </div>
              </div>
              
              <div className="bg-gray-50 rounded-lg p-4">
                <h4 className="text-sm font-medium text-gray-700 mb-2">Summary</h4>
                <p className="text-gray-600">{review.summary || 'No summary available'}</p>
              </div>
              
              {review.key_files && (
                <div className="mt-3">
                  <h4 className="text-sm font-medium text-gray-700 mb-1">Key Files</h4>
                  <p className="text-sm text-gray-600">{review.key_files}</p>
                </div>
              )}
              
              {review.issues && (
                <div className="mt-3">
                  <h4 className="text-sm font-medium text-gray-700 mb-1">Issues</h4>
                  <p className="text-sm text-gray-600 whitespace-pre-line">{review.issues}</p>
                </div>
              )}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}

export default Reviews
