"""CodeSheriff - AI-Powered PR Review Assistant

Main FastAPI application.
"""
import os
import sys
from contextlib import asynccontextmanager
from typing import List, Optional

# Add parent directory to path for imports
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))

from fastapi import FastAPI, HTTPException, Depends, Request
from fastapi.middleware.cors import CORSMiddleware
from fastapi.staticfiles import StaticFiles
from fastapi.templating import Jinja2Templates
from pydantic import BaseModel
from starlette.middleware.sessions import SessionMiddleware

import database
import auth
import github
import pr_review

# Initialize database
database.init_db()


@asynccontextmanager
async def lifespan(app: FastAPI):
    """Application lifespan handler."""
    print("🚀 CodeSheriff starting up...")
    yield
    print("👋 CodeSheriff shutting down...")


# Create FastAPI app
app = FastAPI(
    title="CodeSheriff",
    description="AI-Powered PR Review Assistant",
    version="1.0.0",
    lifespan=lifespan
)

# Session middleware
app.add_middleware(
    SessionMiddleware,
    secret_key=os.getenv("SECRET_KEY", "codesheriff-dev-secret-key-change-in-prod"),
    max_age=86400 * 7  # 7 days
)

# CORS
app.add_middleware(
    CORSMiddleware,
    allow_origins=["http://localhost:5173", "http://localhost:3000"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Include routers
app.include_router(auth.router)
templates = Jinja2Templates(directory=os.path.join(os.path.dirname(__file__), "templates"))


# Pydantic models
class RepoAdd(BaseModel):
    repo_full_name: str


class PRReviewRequest(BaseModel):
    repo_id: int
    pr_number: int
    post_comment: bool = False


class SettingsUpdate(BaseModel):
    auto_post_comment: Optional[bool] = None
    notify_on_review: Optional[bool] = None


# Auth dependency
def get_current_user(request: Request):
    """Get current authenticated user."""
    user_id = request.session.get("user_id")
    if not user_id:
        raise HTTPException(status_code=401, detail="Not authenticated")
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    if not row:
        raise HTTPException(status_code=401, detail="User not found")
    return dict(row)


# Routes
@app.get("/")
async def root(request: Request):
    """Root page."""
    return {"message": "CodeSheriff API", "version": "1.0.0"}


# Repositories
@app.get("/api/repos")
async def get_repos(user = Depends(get_current_user)):
    """Get user's repositories."""
    return database.get_user_repos(user["id"])


@app.post("/api/repos")
async def add_repo(repo: RepoAdd, user = Depends(get_current_user)):
    """Add a repository to monitor."""
    parts = repo.repo_full_name.split('/')
    if len(parts) != 2:
        raise HTTPException(status_code=400, detail="Invalid repo format. Use 'owner/repo'")
    
    owner, repo_name = parts
    
    # Verify repo exists and user has access
    try:
        client = github.get_github_client(user["id"])
        repo_data = client._request("GET", f"https://api.github.com/repos/{owner}/{repo_name}")
    except Exception as e:
        raise HTTPException(status_code=400, detail=f"Cannot access repo: {str(e)}")
    
    repo_id = database.add_repo(user["id"], repo_name, repo.repo_full_name)
    return {"repo_id": repo_id, "message": f"Added {repo.repo_full_name}"}


@app.delete("/api/repos/{repo_id}")
async def delete_repo(repo_id: int, user = Depends(get_current_user)):
    """Remove a repository."""
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("DELETE FROM repos WHERE id = ? AND user_id = ?", (repo_id, user["id"]))
    conn.commit()
    conn.close()
    return {"message": "Repository removed"}


# GitHub Integration
@app.get("/api/github/repos")
async def get_github_repos(user = Depends(get_current_user)):
    """Get repositories from GitHub."""
    try:
        client = github.get_github_client(user["id"])
        repos = client.get_user_repos()
        return [{"name": r["name"], "full_name": r["full_name"], "private": r["private"]} 
                for r in repos]
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


# Pull Requests
@app.get("/api/repos/{repo_id}/pulls")
async def get_repo_pulls(repo_id: int, state: str = "open", user = Depends(get_current_user)):
    """Get pull requests for a repository."""
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM repos WHERE id = ? AND user_id = ?", (repo_id, user["id"]))
    repo = cursor.fetchone()
    conn.close()
    
    if not repo:
        raise HTTPException(status_code=404, detail="Repository not found")
    
    repo = dict(repo)
    owner, repo_name = repo['repo_full_name'].split('/')
    
    try:
        client = github.get_github_client(user["id"])
        pulls = client.get_repo_pulls(owner, repo_name, state)
        return pulls
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


# PR Review
@app.post("/api/review")
async def review_pr(request: PRReviewRequest, user = Depends(get_current_user)):
    """Analyze a pull request."""
    try:
        result = await pr_review.review_pull_request(
            user_id=user["id"],
            repo_id=request.repo_id,
            pr_number=request.pr_number,
            post_comment=request.post_comment
        )
        return result
    except Exception as e:
        raise HTTPException(status_code=400, detail=str(e))


# Reviews History
@app.get("/api/reviews")
async def get_reviews(limit: int = 50, user = Depends(get_current_user)):
    """Get review history."""
    return database.get_pr_reviews(user["id"], limit)


# Dashboard
@app.get("/api/dashboard")
async def get_dashboard(user = Depends(get_current_user)):
    """Get dashboard statistics."""
    return database.get_dashboard_stats(user["id"])


# Settings
@app.get("/api/settings")
async def get_settings(user = Depends(get_current_user)):
    """Get user settings."""
    return database.get_user_settings(user["id"])


@app.post("/api/settings")
async def update_settings(settings: SettingsUpdate, user = Depends(get_current_user)):
    """Update user settings."""
    database.update_user_settings(
        user["id"],
        auto_post_comment=settings.auto_post_comment,
        notify_on_review=settings.notify_on_review
    )
    return database.get_user_settings(user["id"])


# Webhook endpoint (for GitHub webhooks - optional)
@app.post("/api/webhook")
async def github_webhook(request: Request):
    """Handle GitHub webhooks."""
    # Verify webhook signature in production
    payload = await request.json()
    event = request.headers.get("X-GitHub-Event")
    
    if event == "pull_request":
        # Process PR events
        action = payload.get("action")
        pr = payload.get("pull_request", {})
        repo = payload.get("repository", {})
        
        if action in ["opened", "synchronize"]:
            # Auto-review new PRs (optional feature)
            pass
    
    return {"status": "received"}


if __name__ == "__main__":
    import uvicorn
    uvicorn.run(app, host="0.0.0.0", port=8000)
