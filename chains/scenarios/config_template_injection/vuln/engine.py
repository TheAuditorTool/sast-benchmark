"""Configuration template engine -- VULNERABLE variant.

Treats a configuration string as a Jinja2 template and renders it to
produce the final configuration value. An attacker who controls the
config value can embed Jinja2 expressions to reach globals or trigger RCE.

Chain: user config string -> from_string(config_str) -> render() -> RCE
Individual findings: SSTI via config value (critical)
Chain finding: arbitrary code execution via config template injection (critical)
"""
import jinja2


def render_config(config_str: str, context: dict) -> str:
    """Render a configuration value that may contain Jinja2 template syntax."""
    env = jinja2.Environment()
    template = env.from_string(config_str)
    return template.render(**context)
