"""Cache directory utilities -- VULNERABLE variant.

Creates the application cache directory with mode 0o777. Any local user can
modify or replace cache files, causing the loader to serve attacker-controlled
data to legitimate clients without re-fetching from the authoritative source.

Chain: world-writable cache dir -> attacker tampers cache entries -> loader serves poisoned responses
Individual findings: insecure cache directory permissions (CWE-732)
Chain finding: cache poisoning via world-writable cache directory (high)
"""
import os

CACHE_DIR = "/tmp/app_cache"


def setup_cache_dir():
    """Create the cache directory with permissions."""
    os.makedirs(CACHE_DIR, exist_ok=True)
    os.chmod(CACHE_DIR, 0o777)


def cache_path(key: str) -> str:
    """Return the file path for a given cache key."""
    safe_key = key.replace("/", "_")
    return os.path.join(CACHE_DIR, safe_key + ".json")
