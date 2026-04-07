import os
from storage import ensure_upload_dir, resolve_path

def save_upload(filename, data):
    ensure_upload_dir()
    path = resolve_path(filename)
    if path is None:
        return {"error": "Invalid filename"}, 400

    if os.path.exists(path):
        return {"error": "File already exists"}, 409

# vuln-code-snippet start ChainScenario0197B
    with open(path, "wb") as fh:
        fh.write(data)  # vuln-code-snippet target-line ChainScenario0197B
# vuln-code-snippet end ChainScenario0197B

    return {"status": "ok", "path": path}, 201
