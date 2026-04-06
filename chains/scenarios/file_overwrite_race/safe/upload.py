"""File upload logic -- SAFE variant.

Uses os.open() with O_CREAT | O_EXCL flags, which is an atomic kernel operation.
The kernel will raise FileExistsError if the file already exists at the moment
of the syscall -- there is no window between the check and the create.

Chain: concurrent POST /upload -> os.open(O_CREAT|O_EXCL) [atomic] -> write
Individual findings: none -- create-or-fail is a single atomic syscall
Chain finding: none -- concurrent uploads for the same filename correctly fail (CWE-362 mitigated)
"""
import os
from storage import ensure_upload_dir, resolve_path


def save_upload(filename, data):
    """
    Save data to upload directory under filename.

    SAFE: O_CREAT | O_EXCL makes creation atomic.  If two concurrent uploads
    race, exactly one will receive the FileExistsError and return 409.
    """
    ensure_upload_dir()
    path = resolve_path(filename)
    if path is None:
        return {"error": "Invalid filename"}, 400

# vuln-code-snippet start chain_file_overwrite_safe
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    except FileExistsError:
        return {"error": "File already exists"}, 409
    with os.fdopen(fd, "wb") as fh:
        fh.write(data)  # vuln-code-snippet safe-line chain_file_overwrite_safe
# vuln-code-snippet end chain_file_overwrite_safe

    return {"status": "ok", "path": path}, 201
