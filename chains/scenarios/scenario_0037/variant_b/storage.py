import os

CACHE_DIR = "/tmp/app_cache"

def setup_cache_dir():
    os.makedirs(CACHE_DIR, exist_ok=True)
    os.chmod(CACHE_DIR, 0o700)

def cache_path(key: str) -> str:
    safe_key = key.replace("/", "_")
    return os.path.join(CACHE_DIR, safe_key + ".json")
