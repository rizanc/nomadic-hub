import { useState, useEffect } from 'react'

function Settings() {
  const [settings, setSettings] = useState(null)
  const [loading, setLoading] = useState(true)
  const [saving, setSaving] = useState(false)

  useEffect(() => {
    fetchSettings()
  }, [])

  const fetchSettings = async () => {
    try {
      const res = await fetch('/api/settings')
      setSettings(await res.json())
    } catch (e) {
      console.error('Failed to fetch settings:', e)
    } finally {
      setLoading(false)
    }
  }

  const handleToggle = async (key, value) => {
    setSaving(true)
    try {
      const updates = { [key]: value }
      const res = await fetch('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(updates)
      })
      setSettings(await res.json())
    } catch (e) {
      console.error('Failed to update settings:', e)
    } finally {
      setSaving(false)
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
    <div className="max-w-2xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
      <h1 className="text-2xl font-bold text-gray-900 mb-6">Settings</h1>
      
      <div className="card mb-6">
        <h2 className="text-lg font-semibold mb-4">Review Preferences</h2>
        
        <div className="space-y-4">
          <div className="flex justify-between items-center">
            <div>
              <h3 className="font-medium text-gray-900">Auto-post comments</h3>
              <p className="text-sm text-gray-500">
                Automatically post review summary as PR comment
              </p>
            </div>
            <button
              onClick={() => handleToggle('auto_post_comment', !settings?.auto_post_comment)}
              disabled={saving}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                settings?.auto_post_comment ? 'bg-sheriff-600' : 'bg-gray-300'
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings?.auto_post_comment ? 'translate-x-6' : 'translate-x-1'
                }`}
              />
            </button>
          </div>
          
          <div className="flex justify-between items-center">
            <div>
              <h3 className="font-medium text-gray-900">Notifications</h3>
              <p className="text-sm text-gray-500">
                Get notified when reviews are complete
              </p>
            </div>
            <button
              onClick={() => handleToggle('notify_on_review', !settings?.notify_on_review)}
              disabled={saving}
              className={`relative inline-flex h-6 w-11 items-center rounded-full transition-colors ${
                settings?.notify_on_review ? 'bg-sheriff-600' : 'bg-gray-300'
              }`}
            >
              <span
                className={`inline-block h-4 w-4 transform rounded-full bg-white transition-transform ${
                  settings?.notify_on_review ? 'translate-x-6' : 'translate-x-1'
                }`}
              />
            </button>
          </div>
        </div>
      </div>
      
      <div className="card mb-6">
        <h2 className="text-lg font-semibold mb-4">API Configuration</h2>
        <div className="bg-gray-50 rounded-lg p-4">
          <p className="text-sm text-gray-600 mb-3">
            Configure your API keys as environment variables:
          </p>
          <div className="font-mono text-sm bg-gray-800 text-gray-100 p-3 rounded">
            <div># For OpenAI</div>
            <div>export OPENAI_API_KEY=sk-...</div>
            <div className="mt-2"># For Anthropic (alternative)</div>
            <div>export ANTHROPIC_API_KEY=sk-ant-...</div>
            <div className="mt-2"># GitHub OAuth</div>
            <div>export GITHUB_CLIENT_ID=...</div>
            <div>export GITHUB_CLIENT_SECRET=...</div>
          </div>
        </div>
      </div>
      
      <div className="card">
        <h2 className="text-lg font-semibold mb-4">About</h2>
        <div className="text-gray-600">
          <p className="mb-2">
            <strong>CodeSheriff</strong> - AI-Powered PR Review Assistant
          </p>
          <p className="text-sm">
            Version 1.0.0
          </p>
          <p className="text-sm mt-2">
            Uses OpenAI GPT or Anthropic Claude for intelligent code analysis.
          </p>
        </div>
      </div>
    </div>
  )
}

export default Settings
