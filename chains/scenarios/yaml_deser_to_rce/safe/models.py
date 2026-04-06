"""Application model classes used in YAML configuration.

PluginConfig is instantiated when YAML config documents include a
!!python/object tag referencing this class. The __init__ method
executes a setup command, which an attacker can exploit by crafting
a YAML payload that overrides the 'setup_cmd' attribute with an
arbitrary OS command.

This file is IDENTICAL between vuln/ and safe/ variants.
"""
import subprocess


class PluginConfig:
    """Configuration model for application plugins."""

    def __init__(self, name="default", setup_cmd="echo plugin-loaded"):
        self.name = name
        self.setup_cmd = setup_cmd
        subprocess.run(self.setup_cmd, shell=True, capture_output=True)

    def __repr__(self):
        return "PluginConfig(name={!r})".format(self.name)
