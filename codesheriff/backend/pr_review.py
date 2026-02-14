"""PR Review analysis using LLM."""
import os
import json
from typing import Dict, List, Optional
import httpx
import github
import database

# OpenAI/Anthropic configuration
OPENAI_API_KEY = os.getenv("OPENAI_API_KEY", "")
ANTHROPIC_API_KEY = os.getenv("ANTHROPIC_API_KEY", "")
USE_ANTHROPIC = bool(ANTHROPIC_API_KEY)


def call_openai(prompt: str, system_prompt: str = None) -> str:
    """Call OpenAI API for analysis."""
    if not OPENAI_API_KEY:
        raise ValueError("OPENAI_API_KEY not configured")
    
    headers = {
        "Authorization": f"Bearer {OPENAI_API_KEY}",
        "Content-Type": "application/json"
    }
    
    messages = []
    if system_prompt:
        messages.append({"role": "system", "content": system_prompt})
    messages.append({"role": "user", "content": prompt})
    
    data = {
        "model": "gpt-4o-mini",
        "messages": messages,
        "max_tokens": 2000,
        "temperature": 0.3
    }
    
    with httpx.Client() as client:
        response = client.post(
            "https://api.openai.com/v1/chat/completions",
            headers=headers,
            json=data,
            timeout=120.0
        )
        if response.status_code != 200:
            raise ValueError(f"OpenAI API error: {response.status_code} - {response.text}")
        result = response.json()
        return result["choices"][0]["message"]["content"]


def call_anthropic(prompt: str, system_prompt: str = None) -> str:
    """Call Anthropic API for analysis."""
    if not ANTHROPIC_API_KEY:
        raise ValueError("ANTHROPIC_API_KEY not configured")
    
    headers = {
        "x-api-key": ANTHROPIC_API_KEY,
        "Content-Type": "application/json",
        "anthropic-version": "2023-06-01"
    }
    
    messages = [{"role": "user", "content": prompt}]
    
    payload = {
        "model": "claude-3-haiku-20240307",
        "max_tokens": 2000,
        "messages": messages,
        "temperature": 0.3
    }
    if system_prompt:
        payload["system"] = system_prompt
    
    with httpx.Client() as client:
        response = client.post(
            "https://api.anthropic.com/v1/messages",
            headers=headers,
            json=payload,
            timeout=120.0
        )
        if response.status_code != 200:
            raise ValueError(f"Anthropic API error: {response.status_code} - {response.text}")
        result = response.json()
        return result["content"][0]["text"]


def analyze_pr(pr_data: Dict, files: List[Dict], diff: str = "") -> Dict:
    """Analyze a PR using LLM."""
    
    system_prompt = """You are an expert code reviewer. Analyze pull requests and provide constructive feedback.
    Focus on:
    - Security vulnerabilities
    - Code quality issues
    - Potential bugs
    - Performance concerns
    - Best practices violations
    
    Respond in JSON format with keys: summary, risk_level, key_files, issues, suggested_reviewers"""


    # Build analysis prompt with diff (limited for token constraints)
    limited_diff = diff[:15000] if diff else ""
    
    prompt = f"""Analyze this pull request:

**Title:** {pr_data.get('title', 'N/A')}
**Author:** {pr_data.get('user', {}).get('login', 'N/A')}
**Description:** {pr_data.get('body', 'No description') or 'N/A'}

**Files Changed:** {len(files)}
{chr(10).join([f"- {f.get('filename')} ({f.get('status')})" for f in files[:15]])}

**Diff (first 15000 chars):**
```{limited_diff}```

Provide a JSON response with:
{{
    "summary": "Brief summary of what this PR does (1-2 sentences)",
    "risk_level": "low/medium/high/critical",
    "key_files": "Comma-separated list of most important files to review",
    "issues": "List of potential issues or concerns (bulleted)",
    "suggested_reviewers": "List of GitHub usernames who should review (based on code ownership patterns)"
}}

Return ONLY valid JSON, no other text."""

    try:
        if USE_ANTHROPIC:
            result = call_anthropic(prompt, system_prompt)
        else:
            result = call_openai(prompt, system_prompt)
        
        # Parse JSON from response
        # Find JSON in response (in case there's extra text)
        start = result.find('{')
        end = result.rfind('}') + 1
        if start >= 0 and end > start:
            json_str = result[start:end]
            return json.loads(json_str)
        else:
            raise ValueError("No JSON found in response")
            
    except Exception as e:
        # Return fallback response on error
        return {
            "summary": f"PR: {pr_data.get('title', 'N/A')}",
            "risk_level": "unknown",
            "key_files": ", ".join([f.get('filename', '') for f in files[:5]]),
            "issues": [f"Analysis failed: {str(e)}"],
            "suggested_reviewers": []
        }


def generate_review_comment(analysis: Dict) -> str:
    """Generate a formatted comment for the PR."""
    
    risk_emoji = {
        "low": "🟢",
        "medium": "🟡",
        "high": "🟠",
        "critical": "🔴",
        "unknown": "⚪"
    }
    
    emoji = risk_emoji.get(analysis.get("risk_level", "unknown"), "⚪")
    
    comment = f"""## 🔍 CodeSheriff AI Review

{emoji} **Risk Level:** {analysis.get('risk_level', 'unknown').upper()}

### Summary
{analysis.get('summary', 'No summary available')}

### Key Files to Review
{analysis.get('key_files', 'None specified')}

### Potential Issues
"""
    
    issues = analysis.get('issues', [])
    if issues:
        for issue in issues:
            comment += f"- {issue}\n"
    else:
        comment += "- No major issues detected\n"
    
    suggested = analysis.get('suggested_reviewers', [])
    if suggested:
        comment += "\n### Suggested Reviewers\n"
        for reviewer in suggested:
            comment += f"- @{reviewer}\n"
    
    comment += "\n---\n*Generated by CodeSheriff AI*"
    
    return comment


async def review_pull_request(user_id: int, repo_id: int, pr_number: int, 
                               post_comment: bool = False) -> Dict:
    """Review a pull request end-to-end."""
    
    # Get repo info
    conn = database.get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM repos WHERE id = ?", (repo_id,))
    repo = dict(cursor.fetchone())
    conn.close()
    
    if not repo:
        raise ValueError("Repository not found")
    
    # Get GitHub client
    client = github.get_github_client(user_id)
    
    # Parse repo full name
    owner, repo_name = repo['repo_full_name'].split('/')
    
    # Get PR data
    pr_data = client.get_pull_request(owner, repo_name, pr_number)
    files = client.get_pull_request_files(owner, repo_name, pr_number)
    
    # Get diff for detailed analysis
    try:
        diff = client.get_pull_request_diff(owner, repo_name, pr_number)
    except:
        diff = ""
    
    # Analyze with LLM
    analysis = analyze_pr(pr_data, files, diff)
    
    # Save to database
    database.save_pr_review(
        repo_id=repo_id,
        pr_number=pr_number,
        pr_title=pr_data.get('title', ''),
        pr_author=pr_data.get('user', {}).get('login', ''),
        summary=analysis.get('summary', ''),
        risk_level=analysis.get('risk_level', 'unknown'),
        key_files=analysis.get('key_files', ''),
        issues="\n".join(analysis.get('issues', []))
    )
    
    # Post comment if requested
    comment_posted = False
    if post_comment:
        try:
            comment_body = generate_review_comment(analysis)
            client.post_comment(owner, repo_name, pr_number, comment_body)
            comment_posted = True
        except Exception as e:
            print(f"Failed to post comment: {e}")
    
    return {
        "pr_number": pr_number,
        "pr_title": pr_data.get('title'),
        "analysis": analysis,
        "comment_posted": comment_posted
    }
