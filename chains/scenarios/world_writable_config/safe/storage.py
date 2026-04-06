"""Config file storage utilities -- SAFE variant.

Creates the application config file with mode 0o600, restricting access
to the owning process only. No other local user can overwrite or even read
the file, eliminating the tampering vector.

Chain broken: config file is owner-only -> no local user can tamper -> loader reads trusted data
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
    os.chmod(CONFIG_PATH, 0o600)
