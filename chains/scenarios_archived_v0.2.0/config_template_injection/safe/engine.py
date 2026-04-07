"""Configuration template engine -- SAFE variant.

Returns the configuration string as plain text. No template rendering
is performed on user-supplied values, so injected Jinja2 syntax cannot
execute.

Chain broken: config value is returned as literal text -> no template rendering -> no RCE
"""


def render_config(config_str: str, context: dict) -> str:
    """Return the configuration string as plain text without template expansion."""
    return config_str
