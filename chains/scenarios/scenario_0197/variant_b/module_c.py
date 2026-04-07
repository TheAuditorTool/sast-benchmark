import os
from module_b import ensure_upload_dir, resolve_path

def save_upload(filename, data):
    ensure_upload_dir()
    path = resolve_path(filename)
    if path is None:
        return {"error": "Invalid filename"}, 400

    if os.path.exists(path):
        return {"error": "File already exists"}, 409

    with open(path, "wb") as fh:
        fh.write(data)

    return {"status": "ok", "path": path}, 201
