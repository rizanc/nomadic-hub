"""GitHub API client for fetching PR data."""
import httpx
from typing import List, Dict, Optional, Any
import database

GITHUB_API_BASE = "https://api.github.com"


class GitHubClient:
    """GitHub API client."""
    
    def __init__(self, access_token: str):
        self.access_token = access_token
        self.headers = {
            "Authorization": f"token {access_token}",
            "Accept": "application/vnd.github.v3+json"
        }
    
    def _request(self, method: str, url: str, **kwargs) -> Dict:
        """Make API request."""
        with httpx.Client() as client:
            response = client.request(method, url, headers=self.headers, **kwargs)
            if response.status_code == 404:
                raise ValueError("Resource not found")
            if response.status_code == 403:
                raise ValueError("Rate limit exceeded or forbidden")
            if response.status_code >= 400:
                raise ValueError(f"API error: {response.status_code}")
            return response.json()
    
    def get_user_repos(self) -> List[Dict]:
        """Get user's repositories."""
        repos = []
        page = 1
        while True:
            url = f"{GITHUB_API_BASE}/user/repos?page={page}&per_page=100&sort=updated"
            data = self._request("GET", url)
            if not data:
                break
            repos.extend(data)
            if len(data) < 100:
                break
            page += 1
        return repos
    
    def get_repo_pulls(self, owner: str, repo: str, state: str = "open") -> List[Dict]:
        """Get pull requests for a repository."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/pulls?state={state}&per_page=50"
        return self._request("GET", url)
    
    def get_pull_request(self, owner: str, repo: str, pr_number: int) -> Dict:
        """Get a specific PR."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/pulls/{pr_number}"
        return self._request("GET", url)
    
    def get_pull_request_files(self, owner: str, repo: str, pr_number: int) -> List[Dict]:
        """Get files changed in a PR."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/pulls/{pr_number}/files?per_page=100"
        return self._request("GET", url)
    
    def get_pull_request_diff(self, owner: str, repo: str, pr_number: int) -> str:
        """Get raw diff of a PR."""
        headers = self.headers.copy()
        headers["Accept"] = "application/vnd.github.v3.diff"
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/pulls/{pr_number}"
        with httpx.Client() as client:
            response = client.get(url, headers=headers)
            if response.status_code != 200:
                raise ValueError(f"Failed to get diff: {response.status_code}")
            return response.text
    
    def get_file_content(self, owner: str, repo: str, path: str, ref: str = "main") -> str:
        """Get file content from repository."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/contents/{path}?ref={ref}"
        data = self._request("GET", url)
        import base64
        if data.get("encoding") == "base64":
            return base64.b64decode(data["content"]).decode("utf-8")
        return data.get("content", "")
    
    def get_repo_contributors(self, owner: str, repo: str) -> List[Dict]:
        """Get contributors to a repository."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/contributors?per_page=20"
        return self._request("GET", url)
    
    def get_code_owners(self, owner: str, repo: str, ref: str = "main") -> Optional[Dict]:
        """Try to get CODEOWNERS file."""
        try:
            content = self.get_file_content(owner, repo, "CODEOWNERS", ref)
            return parse_codeowners(content)
        except:
            return None
    
    def post_comment(self, owner: str, repo: str, pr_number: int, body: str) -> Dict:
        """Post a comment on a PR."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/issues/{pr_number}/comments"
        return self._request("POST", url, json={"body": body})
    
    def get_pr_comments(self, owner: str, repo: str, pr_number: int) -> List[Dict]:
        """Get comments on a PR."""
        url = f"{GITHUB_API_BASE}/repos/{owner}/{repo}/issues/{pr_number}/comments"
        return self._request("GET", url)


def parse_codeowners(content: str) -> Dict[str, List[str]]:
    """Parse CODEOWNERS file."""
    owners = {}
    for line in content.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        if len(parts) >= 2:
            path = parts[0]
            team_members = parts[1:]
            owners[path] = team_members
    return owners


def get_github_client(user_id: int) -> GitHubClient:
    """Get GitHub client for user."""
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT access_token FROM users WHERE id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    
    if not row or not row[0]:
        raise ValueError("No GitHub token found")
    
    return GitHubClient(row[0])


def format_pr_for_analysis(pr_data: Dict, files: List[Dict]) -> str:
    """Format PR data for LLM analysis."""
    output = f"""# Pull Request Analysis

## PR Info
- **Title:** {pr_data.get('title', 'N/A')}
- **Number:** {pr_data.get('number', 'N/A')}
- **Author:** {pr_data.get('user', {}).get('login', 'N/A')}
- **Description:** {pr_data.get('body', 'No description') or 'No description'}

## Files Changed ({len(files)} files)
"""
    for f in files:
        output += f"- `{f.get('filename', 'unknown')}`: {f.get('status', 'modified')} (+{f.get('additions', 0)} -{f.get('deletions', 0)})\n"
    
    output += "\n## File Details\n"
    for f in files[:10]:  # Limit to first 10 files for token limits
        filename = f.get('filename', 'unknown')
        patch = f.get('patch', '')
        if patch:
            output += f"\n### {filename}\n```diff\n{patch[:5000]}\n```\n"
    
    return output
