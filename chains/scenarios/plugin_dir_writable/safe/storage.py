"""Plugin directory utilities -- SAFE variant.

Creates the plugin directory with mode 0o700, restricting write access to
the owning process. No local attacker can add files to the directory, so the
loader can only import plugins placed there by the application itself.

Chain broken: plugin dir is owner-only -> no external files added -> loader only executes trusted plugins
"""
import os

PLUGIN_DIR = "/tmp/app_plugins"


def setup_plugin_dir():
    """Create the plugin directory with permissions."""
    os.makedirs(PLUGIN_DIR, exist_ok=True)
    os.chmod(PLUGIN_DIR, 0o700)
