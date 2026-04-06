"""Data layer for the document management service.

This file is IDENTICAL between vuln/ and safe/ variants.
Documents are owned by individual users.  The owner_id column
records which user uploaded the file.
"""
import sqlite3
import os

DB_PATH = "app.db"
STORAGE_ROOT = "/var/app/documents"


def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn


def get_document_metadata(file_id):
    """Return document metadata row, or None if not found."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, owner_id, filename, content_type, size_bytes, created_at FROM documents WHERE id = ?",
        (file_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None


def get_document_path(file_id):
    """Return the filesystem path for the given document id."""
    meta = get_document_metadata(file_id)
    if meta is None:
        return None
    return os.path.join(STORAGE_ROOT, str(file_id), meta["filename"])


def list_user_documents(user_id):
    """Return all documents owned by user_id."""
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, filename, size_bytes, created_at FROM documents WHERE owner_id = ?",
        (user_id,),
    )
    rows = cursor.fetchall()
    conn.close()
    return [dict(r) for r in rows]
