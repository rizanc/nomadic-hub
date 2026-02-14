"""GitHub OAuth authentication."""
import os
import secrets
from typing import Optional, Dict
from fastapi import APIRouter, Request, HTTPException, Depends
from fastapi.responses import JSONResponse, RedirectResponse
import httpx
import database

router = APIRouter(prefix="/auth", tags=["auth"])

# GitHub OAuth config - set these in environment
GITHUB_CLIENT_ID = os.getenv("GITHUB_CLIENT_ID", "your_client_id_here")
GITHUB_CLIENT_SECRET = os.getenv("GITHUB_CLIENT_SECRET", "your_client_secret_here")
GITHUB_REDIRECT_URI = os.getenv("GITHUB_REDIRECT_URI", "http://localhost:8000/auth/callback")

# In-memory state storage (use Redis in production)
oauth_states = {}


def get_github_token(code: str) -> Dict:
    """Exchange code for access token."""
    token_url = "https://github.com/login/oauth/access_token"
    headers = {"Accept": "application/json"}
    data = {
        "client_id": GITHUB_CLIENT_ID,
        "client_secret": GITHUB_CLIENT_SECRET,
        "code": code
    }
    
    with httpx.Client() as client:
        response = client.post(token_url, json=data, headers=headers)
        if response.status_code != 200:
            raise HTTPException(status_code=400, detail="Failed to get access token")
        return response.json()


def get_github_user(access_token: str) -> Dict:
    """Get current user from GitHub API."""
    headers = {"Authorization": f"token {access_token}"}
    with httpx.Client() as client:
        response = client.get("https://api.github.com/user", headers=headers)
        if response.status_code != 200:
            raise HTTPException(status_code=400, detail="Failed to get user info")
        return response.json()


def get_current_user(request: Request) -> Optional[Dict]:
    """Get current authenticated user from session."""
    user_id = request.session.get("user_id")
    if not user_id:
        return None
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


@router.get("/login")
async def login():
    """Initiate GitHub OAuth flow."""
    state = secrets.token_urlsafe(32)
    oauth_states[state] = True  # Store state
    
    github_auth_url = (
        f"https://github.com/login/oauth/authorize"
        f"?client_id={GITHUB_CLIENT_ID}"
        f"&redirect_uri={GITHUB_REDIRECT_URI}"
        f"&scope=repo,read:user"
        f"&state={state}"
    )
    return RedirectResponse(github_auth_url)


@router.get("/callback")
async def callback(code: str, state: str, request: Request):
    """Handle OAuth callback."""
    # Verify state
    if state not in oauth_states:
        raise HTTPException(status_code=400, detail="Invalid state")
    del oauth_states[state]
    
    # Get access token
    token_data = get_github_token(code)
    access_token = token_data.get("access_token")
    
    if not access_token:
        raise HTTPException(status_code=400, detail="No access token received")
    
    # Get user info
    github_user = get_github_user(access_token)
    github_id = str(github_user["id"])
    username = github_user["login"]
    
    # Create or update user in database
    user_id = database.create_user(github_id, username, access_token)
    
    # Store user info in session
    request.session["user_id"] = user_id
    request.session["username"] = username
    
    # Redirect to frontend
    frontend_url = os.getenv("FRONTEND_URL", "http://localhost:5173")
    return RedirectResponse(f"{frontend_url}/dashboard")


@router.get("/me")
async def get_me(request: Request):
    """Get current user info."""
    user = get_current_user(request)
    if not user:
        raise HTTPException(status_code=401, detail="Not authenticated")
    return {"id": user["id"], "username": user["username"]}


@router.post("/logout")
async def logout(request: Request):
    """Logout user."""
    request.session.clear()
    return {"message": "Logged out successfully"}


@router.get("/status")
async def auth_status(request: Request):
    """Check authentication status."""
    user = get_current_user(request)
    if user:
        return {"authenticated": True, "username": user["username"]}
    return {"authenticated": False}
