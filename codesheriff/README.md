# CodeSheriff 🦸

AI-Powered Pull Request Review Assistant

## Features

- **GitHub OAuth Login** - Secure authentication with GitHub
- **Repository Management** - Connect and monitor multiple repositories
- **AI-Powered Reviews** - Analyzes PRs using OpenAI or Anthropic
- **Review Summary** - Generates summary, risk level, key files, and issues
- **Auto-Post Comments** - Optionally post reviews directly to PRs
- **Dashboard** - View statistics and review history

## Prerequisites

- Python 3.9+
- Node.js 18+
- GitHub OAuth App (see setup below)

## Quick Start

### 1. Create GitHub OAuth App

1. Go to GitHub Settings → Developer settings → OAuth Apps
2. Click "New OAuth App"
3. Fill in details:
   - **Application name**: CodeSheriff
   - **Homepage URL**: http://localhost:5173
   - **Authorization callback URL**: http://localhost:8000/auth/callback
4. Save your `Client ID` and generate a `Client Secret`

### 2. Clone/Setup

```bash
cd /Users/costinrizan/clawd/codesheriff
```

### 3. Configure Environment Variables

Create a `.env` file in `/backend`:

```bash
# Required - GitHub OAuth
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
GITHUB_REDIRECT_URI=http://localhost:8000/auth/callback

# Required - AI Provider (choose one)
OPENAI_API_KEY=sk-...      # For OpenAI GPT-4
# OR
ANTHROPIC_API_KEY=sk-ant-...  # For Anthropic Claude

# Optional
FRONTEND_URL=http://localhost:5173
SECRET_KEY=your-random-secret-key
```

### 4. Install Backend Dependencies

```bash
cd backend
pip install -r requirements.txt
```

### 5. Install Frontend Dependencies

```bash
cd frontend
npm install
```

### 6. Start the Application

**Terminal 1 - Backend:**
```bash
cd backend
python main.py
# Or: uvicorn main:app --reload --port 8000
```

**Terminal 2 - Frontend:**
```bash
cd frontend
npm run dev
```

### 7. Access the App

- Frontend: http://localhost:5173
- Backend API: http://localhost:8000

## Project Structure

```
codesheriff/
├── backend/
│   ├── main.py          # FastAPI app & routes
│   ├── auth.py          # GitHub OAuth handlers
│   ├── github.py        # GitHub API client
│   ├── pr_review.py     # LLM-powered PR analysis
│   ├── database.py      # SQLite database
│   └── requirements.txt
├── frontend/
│   ├── src/
│   │   ├── App.jsx      # Main React app
│   │   ├── components/
│   │   │   ├── Login.jsx
│   │   │   ├── Dashboard.jsx
│   │   │   ├── Repos.jsx
│   │   │   ├── Reviews.jsx
│   │   │   ├── Settings.jsx
│   │   │   └── Navbar.jsx
│   │   └── index.css
│   ├── package.json
│   └── vite.config.js
└── README.md
```

## Usage

1. **Login**: Click "Continue with GitHub" to authenticate
2. **Add Repositories**: Go to Repositories → Add your `owner/repo`
3. **View PRs**: Click "View PRs" on any repository
4. **Review**: Click "Review" to analyze a PR, or "Post Comment" to add analysis to the PR
5. **Dashboard**: Check statistics and review history

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | `/auth/login` | Start OAuth flow |
| GET | `/auth/callback` | OAuth callback |
| GET | `/auth/status` | Check auth status |
| GET | `/api/repos` | List connected repos |
| POST | `/api/repos` | Add a repository |
| DELETE | `/api/repos/{id}` | Remove repository |
| GET | `/api/repos/{id}/pulls` | Get PRs for repo |
| POST | `/api/review` | Analyze a PR |
| GET | `/api/reviews` | Get review history |
| GET | `/api/dashboard` | Get dashboard stats |
| GET/POST | `/api/settings` | User settings |

## Production Deployment

### Environment Variables for Production

```bash
# Must be set for production
GITHUB_CLIENT_ID=...
GITHUB_CLIENT_SECRET=...
OPENAI_API_KEY=...  # or ANTHROPIC_API_KEY
SECRET_KEY=<generate-random-64-chars>
FRONTEND_URL=https://your-domain.com

# For production, use PostgreSQL instead of SQLite
DATABASE_URL=postgresql://user:pass@localhost/codesheriff
```

### Docker (Future)

```dockerfile
# Coming soon
```

## License

MIT License
