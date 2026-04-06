"""File upload logic -- VULNERABLE variant.

Calls os.path.exists() to guard against overwriting an existing file, then
opens the path for writing in a separate call.  A concurrent upload with the
same filename passes the exists() check (file not yet present) before either
write commits, and whichever finishes second silently overwrites the first.

Chain: concurrent POST /upload -> os.path.exists() -> [race window] -> open(path, 'wb')
Individual findings: TOCTOU on file existence check (medium)
Chain finding: concurrent upload silently overwrites another user's file (CWE-362, critical)
"""
import os
from storage import ensure_upload_dir, resolve_path


def save_upload(filename, data):
    """
    Save data to upload directory under filename.

    VULNERABLE: exists() check and open() are two separate syscalls.
    Between them another process can create the file, making the check stale.
    """
    ensure_upload_dir()
    path = resolve_path(filename)
    if path is None:
        return {"error": "Invalid filename"}, 400

    # TOCTOU: between this check and the open() call another upload can land
    if os.path.exists(path):
        return {"error": "File already exists"}, 409

# vuln-code-snippet start chain_file_overwrite_vuln
    with open(path, "wb") as fh:
        fh.write(data)  # vuln-code-snippet vuln-line chain_file_overwrite_vuln
# vuln-code-snippet end chain_file_overwrite_vuln

    return {"status": "ok", "path": path}, 201
