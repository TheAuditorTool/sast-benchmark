"""Cache directory utilities -- SAFE variant.

Creates the cache directory with mode 0o700, restricting write access to the
owning process. No local attacker can modify or replace cache files, so the
loader always serves content written by the application itself.

Chain broken: cache dir is owner-only -> no external modifications -> loader serves authentic data
"""
import os

CACHE_DIR = "/tmp/app_cache"


def setup_cache_dir():
    """Create the cache directory with permissions."""
    os.makedirs(CACHE_DIR, exist_ok=True)
    os.chmod(CACHE_DIR, 0o700)


def cache_path(key: str) -> str:
    """Return the file path for a given cache key."""
    safe_key = key.replace("/", "_")
    return os.path.join(CACHE_DIR, safe_key + ".json")
