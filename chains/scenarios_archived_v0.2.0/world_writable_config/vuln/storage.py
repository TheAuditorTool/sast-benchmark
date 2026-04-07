"""Config file storage utilities -- VULNERABLE variant.

Creates the application config file with mode 0o666, making it
world-writable. Any local user or compromised process can overwrite
the config, injecting arbitrary values that the loader then trusts.

Chain: world-writable config written -> loader reads it and trusts values -> tampered config executed
Individual findings: insecure file permissions (CWE-732)
Chain finding: config tampering via world-writable file (critical)
"""
import os
import json


CONFIG_PATH = "/tmp/app_config.json"


def write_default_config():
    """Write the default application configuration to disk."""
    defaults = {
        "debug": False,
        "allowed_hosts": ["localhost"],
        "db_url": "sqlite:///app.db",
    }
    with open(CONFIG_PATH, "w") as fh:
        json.dump(defaults, fh)
    os.chmod(CONFIG_PATH, 0o666)
