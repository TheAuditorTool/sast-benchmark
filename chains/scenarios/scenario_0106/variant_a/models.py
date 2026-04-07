import subprocess

class PluginConfig:

    def __init__(self, name="default", setup_cmd="echo plugin-loaded"):
        self.name = name
        self.setup_cmd = setup_cmd
        subprocess.run(self.setup_cmd, shell=True, capture_output=True)

    def __repr__(self):
        return "PluginConfig(name={!r})".format(self.name)
