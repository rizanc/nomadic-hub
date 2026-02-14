function Login() {
  const handleLogin = () => {
    window.location.href = '/auth/login'
  }

  return (
    <div className="min-h-screen flex items-center justify-center bg-gradient-to-br from-sheriff-50 to-sheriff-100">
      <div className="max-w-md w-full p-8">
        <div className="text-center mb-8">
          <div className="text-6xl mb-4">🦸</div>
          <h1 className="text-4xl font-bold text-sheriff-800 mb-2">CodeSheriff</h1>
          <p className="text-gray-600 text-lg">AI-Powered PR Reviews</p>
        </div>
        
        <div className="card">
          <h2 className="text-xl font-semibold mb-4">Sign in with GitHub</h2>
          <p className="text-gray-600 mb-6">
            Connect your GitHub account to start reviewing pull requests with AI assistance.
          </p>
          
          <button
            onClick={handleLogin}
            className="w-full btn btn-primary flex items-center justify-center gap-2 py-3"
          >
            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 24 24">
              <path d="M12 0C5.37 0 0 5.37 0 12c0 5.31 3.435 9.795 8.205 11.385.6.105.825-.255.825-.57 0-.285-.015-1.23-.015-2.235-3.015.555-3.795-.735-4.035-1.41-.135-.345-.72-1.41-1.23-1.695-.42-.225-1.02-.78-.015-.795.945-.015 1.62.87 1.845 1.23 1.08 1.815 2.805 1.305 3.495.99.105-.78.42-1.305.765-1.605-2.67-.3-5.46-1.335-5.46-5.925 0-1.305.465-2.385 1.23-3.225-.12-.3-.54-1.53.12-3.18 0 0 1.005-.315 3.3 1.23.96-.27 1.98-.405 3-.405s2.04.135 3 .405c2.295-1.56 3.3-1.23 3.3-1.23.66 1.65.24 2.88.12 3.18.765.84 1.23 1.905 1.23 3.225 0 4.605-2.805 5.625-5.475 5.925.435.375.81 1.095.81 2.22 0 1.605-.015 2.895-.015 3.3 0 .315.225.69.825.57A12.02 12.02 0 0024 12c0-6.63-5.37-12-12-12z"/>
            </svg>
            Continue with GitHub
          </button>
        </div>
        
        <div className="mt-6 text-center text-sm text-gray-500">
          <p>CodeSheriff will request access to:</p>
          <ul className="mt-2 space-y-1">
            <li>• Read your repositories</li>
            <li>• Read pull request metadata</li>
            <li>• Post comments on pull requests</li>
          </ul>
        </div>
      </div>
    </div>
  )
}

export default Login
