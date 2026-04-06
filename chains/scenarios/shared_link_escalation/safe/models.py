"""Data layer for the shared-link document service.

This file is IDENTICAL between vuln/ and safe/ variants.
Documents can be shared via a token.  The share record stores the
access_level ('view' or 'edit') and the target document_id.
"""
import sqlite3
import secrets

DB_PATH = "app.db"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_share_by_token(token):
    """Return the share record for the given token, or None."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, document_id, access_level, expires_at FROM shares WHERE token = ?",
        (token,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


def get_document(document_id):
    """Return document metadata and content."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, owner_id, title, body, updated_at FROM documents WHERE id = ?",
        (document_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


def update_document_body(document_id, new_body):
    """Overwrite a document's body text."""
    conn = get_db()
    conn.execute(
        "UPDATE documents SET body = ?, updated_at = datetime('now') WHERE id = ?",
        (new_body, document_id),
    )
    conn.commit()
    conn.close()
