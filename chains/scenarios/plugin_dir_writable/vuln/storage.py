"""Plugin directory utilities -- VULNERABLE variant.

Creates the plugin directory with mode 0o777, allowing any local user to
add Python files. When the loader imports all .py files from this directory,
an attacker-planted plugin is executed with application privileges.

Chain: world-writable plugin dir -> attacker plants malicious .py -> loader imports it -> arbitrary code runs
Individual findings: insecure plugin directory permissions (CWE-732)
Chain finding: code execution via plugin directory injection (critical)
"""
import os

PLUGIN_DIR = "/tmp/app_plugins"


def setup_plugin_dir():
    """Create the plugin directory with permissions."""
    os.makedirs(PLUGIN_DIR, exist_ok=True)
    os.chmod(PLUGIN_DIR, 0o777)
