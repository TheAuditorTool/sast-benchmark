import os
from storage import ensure_upload_dir, resolve_path

def save_upload(filename, data):
    ensure_upload_dir()
    path = resolve_path(filename)
    if path is None:
        return {"error": "Invalid filename"}, 400

# vuln-code-snippet start ChainScenario0197A
    try:
        fd = os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o644)
    except FileExistsError:
        return {"error": "File already exists"}, 409
    with os.fdopen(fd, "wb") as fh:
        fh.write(data)  # vuln-code-snippet target-line ChainScenario0197A
# vuln-code-snippet end ChainScenario0197A

    return {"status": "ok", "path": path}, 201
