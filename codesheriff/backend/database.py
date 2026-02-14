"""Database models and operations for CodeSheriff."""
import sqlite3
from datetime import datetime
from typing import Optional, List, Dict, Any
import os

DB_PATH = os.path.join(os.path.dirname(os.path.dirname(__file__)), "codesheriff.db")


def get_db():
    """Get database connection."""
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def init_db():
    """Initialize database tables."""
    conn = get_db()
    cursor = conn.cursor()
    
    # Users table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            github_id TEXT UNIQUE NOT NULL,
            username TEXT NOT NULL,
            access_token TEXT,
            refresh_token TEXT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    """)
    
    # Repositories table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS repos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            repo_name TEXT NOT NULL,
            repo_full_name TEXT NOT NULL,
            enabled BOOLEAN DEFAULT TRUE,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (user_id) REFERENCES users(id),
            UNIQUE(user_id, repo_full_name)
        )
    """)
    
    # PR Reviews table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS pr_reviews (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            repo_id INTEGER NOT NULL,
            pr_number INTEGER NOT NULL,
            pr_title TEXT,
            pr_author TEXT,
            summary TEXT,
            risk_level TEXT,
            key_files TEXT,
            issues TEXT,
            status TEXT DEFAULT 'pending',
            reviewed_at TIMESTAMP,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
            FOREIGN KEY (repo_id) REFERENCES repos(id),
            UNIQUE(repo_id, pr_number)
        )
    """)
    
    # Settings table
    cursor.execute("""
        CREATE TABLE IF NOT EXISTS settings (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            user_id INTEGER NOT NULL,
            auto_post_comment BOOLEAN DEFAULT FALSE,
            notify_on_review BOOLEAN DEFAULT TRUE,
            FOREIGN KEY (user_id) REFERENCES users(id)
        )
    """)
    
    conn.commit()
    conn.close()


def create_user(github_id: str, username: str, access_token: str = None) -> int:
    """Create or update a user."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("""
        INSERT INTO users (github_id, username, access_token)
        VALUES (?, ?, ?)
        ON CONFLICT(github_id) DO UPDATE SET
            username = excluded.username,
            access_token = COALESCE(excluded.access_token, users.access_token)
    """, (github_id, username, access_token))
    conn.commit()
    user_id = cursor.lastrowid
    if user_id == 0:
        cursor.execute("SELECT id FROM users WHERE github_id = ?", (github_id,))
        user_id = cursor.fetchone()[0]
    conn.close()
    return user_id


def get_user_by_github_id(github_id: str) -> Optional[Dict]:
    """Get user by GitHub ID."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM users WHERE github_id = ?", (github_id,))
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


def add_repo(user_id: int, repo_name: str, repo_full_name: str) -> int:
    """Add a repository for a user."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("""
        INSERT OR IGNORE INTO repos (user_id, repo_name, repo_full_name)
        VALUES (?, ?, ?)
    """, (user_id, repo_name, repo_full_name))
    conn.commit()
    cursor.execute("SELECT id FROM repos WHERE user_id = ? AND repo_full_name = ?", 
                   (user_id, repo_full_name))
    repo_id = cursor.fetchone()[0]
    conn.close()
    return repo_id


def get_user_repos(user_id: int) -> List[Dict]:
    """Get all repos for a user."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM repos WHERE user_id = ?", (user_id,))
    rows = cursor.fetchall()
    conn.close()
    return [dict(row) for row in rows]


def save_pr_review(repo_id: int, pr_number: int, pr_title: str, pr_author: str,
                   summary: str, risk_level: str, key_files: str, issues: str) -> int:
    """Save a PR review."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("""
        INSERT INTO pr_reviews (repo_id, pr_number, pr_title, pr_author, 
                                summary, risk_level, key_files, issues, status, reviewed_at)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, 'completed', ?)
        ON CONFLICT(repo_id, pr_number) DO UPDATE SET
            summary = excluded.summary,
            risk_level = excluded.risk_level,
            key_files = excluded.key_files,
            issues = excluded.issues,
            status = 'completed',
            reviewed_at = excluded.reviewed_at
    """, (repo_id, pr_number, pr_title, pr_author, summary, risk_level, key_files, 
          issues, datetime.utcnow().isoformat()))
    conn.commit()
    conn.close()
    return pr_number


def get_pr_reviews(user_id: int, limit: int = 50) -> List[Dict]:
    """Get PR reviews for user's repos."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("""
        SELECT pr.*, r.repo_full_name 
        FROM pr_reviews pr
        JOIN repos r ON pr.repo_id = r.id
        WHERE r.user_id = ?
        ORDER BY pr.created_at DESC
        LIMIT ?
    """, (user_id, limit))
    rows = cursor.fetchall()
    conn.close()
    return [dict(row) for row in rows]


def get_dashboard_stats(user_id: int) -> Dict[str, Any]:
    """Get dashboard statistics."""
    conn = get_db()
    cursor = conn.cursor()
    
    # Total PRs reviewed
    cursor.execute("""
        SELECT COUNT(*) FROM pr_reviews pr
        JOIN repos r ON pr.repo_id = r.id
        WHERE r.user_id = ? AND pr.status = 'completed'
    """, (user_id,))
    total_reviews = cursor.fetchone()[0]
    
    # Risk level distribution
    cursor.execute("""
        SELECT pr.risk_level, COUNT(*) as count FROM pr_reviews pr
        JOIN repos r ON pr.repo_id = r.id
        WHERE r.user_id = ? AND pr.status = 'completed'
        GROUP BY pr.risk_level
    """, (user_id,))
    risk_dist = {row[0]: row[1] for row in cursor.fetchall()}
    
    # Recent reviews (last 7 days)
    cursor.execute("""
        SELECT COUNT(*) FROM pr_reviews pr
        JOIN repos r ON pr.repo_id = r.id
        WHERE r.user_id = ? AND pr.status = 'completed'
        AND pr.created_at >= datetime('now', '-7 days')
    """, (user_id,))
    recent_reviews = cursor.fetchone()[0]
    
    # Common issues
    cursor.execute("""
        SELECT pr.issues FROM pr_reviews pr
        JOIN repos r ON pr.repo_id = r.id
        WHERE r.user_id = ? AND pr.status = 'completed' AND pr.issues IS NOT NULL
    """, (user_id,))
    all_issues = [row[0] for row in cursor.fetchall() if row[0]]
    
    conn.close()
    
    return {
        "total_reviews": total_reviews,
        "risk_distribution": risk_dist,
        "recent_reviews": recent_reviews,
        "common_issues": all_issues[:10]
    }


def get_user_settings(user_id: int) -> Dict[str, Any]:
    """Get user settings."""
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("SELECT * FROM settings WHERE user_id = ?", (user_id,))
    row = cursor.fetchone()
    conn.close()
    if row:
        return dict(row)
    # Create default settings
    conn = get_db()
    cursor = conn.cursor()
    cursor.execute("""
        INSERT INTO settings (user_id, auto_post_comment, notify_on_review)
        VALUES (?, FALSE, TRUE)
    """, (user_id,))
    conn.commit()
    conn.close()
    return {"user_id": user_id, "auto_post_comment": False, "notify_on_review": True}


def update_user_settings(user_id: int, auto_post_comment: bool = None, 
                         notify_on_review: bool = None) -> bool:
    """Update user settings."""
    conn = get_db()
    cursor = conn.cursor()
    if auto_post_comment is not None:
        cursor.execute("UPDATE settings SET auto_post_comment = ? WHERE user_id = ?",
                      (auto_post_comment, user_id))
    if notify_on_review is not None:
        cursor.execute("UPDATE settings SET notify_on_review = ? WHERE user_id = ?",
                      (notify_on_review, user_id))
    conn.commit()
    conn.close()
    return True
