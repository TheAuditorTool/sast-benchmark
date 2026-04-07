import sqlite3
import secrets

DB_PATH = "app.db"

def get_db():
    conn = sqlite3.connect(DB_PATH)
    conn.row_factory = sqlite3.Row
    return conn

def get_share_by_token(token):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, document_id, access_level, expires_at FROM shares WHERE token = ?",
        (token,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None

def get_document(document_id):
    conn = get_db()
    cursor = conn.execute(
        "SELECT id, owner_id, title, body, updated_at FROM documents WHERE id = ?",
        (document_id,),
    )
    row = cursor.fetchone()
    conn.close()
    return dict(row) if row else None

def update_document_body(document_id, new_body):
    conn = get_db()
    conn.execute(
        "UPDATE documents SET body = ?, updated_at = datetime('now') WHERE id = ?",
        (new_body, document_id),
    )
    conn.commit()
    conn.close()
