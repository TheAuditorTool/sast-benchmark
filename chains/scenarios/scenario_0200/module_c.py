import os

PLUGIN_DIR = "/tmp/app_plugins"

def setup_plugin_dir():
    os.makedirs(PLUGIN_DIR, exist_ok=True)
    os.chmod(PLUGIN_DIR, 0o700)
