import os
import json

CONFIG_PATH = "/tmp/app_config.json"

def write_default_config():
    defaults = {
        "debug": False,
        "allowed_hosts": ["localhost"],
        "db_url": "sqlite:///app.db",
    }
    with open(CONFIG_PATH, "w") as fh:
        json.dump(defaults, fh)
    os.chmod(CONFIG_PATH, 0o666)
