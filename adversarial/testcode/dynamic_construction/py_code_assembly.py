"""
Dynamic Construction Test Cases (Python)
Tests SAST tool ability to detect obfuscated code assembly.
"""
import base64


# ============================================================================
# VULNERABLE: bytes.fromhex reconstructs "exec"
# ============================================================================

# vuln-code-snippet start fromhex_exec
def stealth_exec(payload):
    # 65=e, 78=x, 65=e, 63=c
    fn_name = bytes.fromhex("65786563").decode("utf-8")
    getattr(__builtins__, fn_name)(payload)  # vuln-code-snippet vuln-line fromhex_exec
# vuln-code-snippet end fromhex_exec


# ============================================================================
# VULNERABLE: getattr on builtins with string concatenation
# ============================================================================

# vuln-code-snippet start getattr_builtins
def indirect_exec(code):
    part1 = "ex"
    part2 = "ec"
    fn = getattr(__builtins__, part1 + part2)
    fn(code)  # vuln-code-snippet vuln-line getattr_builtins
# vuln-code-snippet end getattr_builtins


# ============================================================================
# VULNERABLE: Double base64 encoding chain to exec
# ============================================================================

# vuln-code-snippet start base64_chain
def double_decode_exec(encoded):
    # Layer 1: base64 decode
    layer1 = base64.b64decode(encoded)
    # Layer 2: base64 decode again
    payload = base64.b64decode(layer1).decode("utf-8")
    exec(payload)  # vuln-code-snippet vuln-line base64_chain
# vuln-code-snippet end base64_chain


# ============================================================================
# SAFE: bytes.fromhex for CSS color parsing (legitimate)
# ============================================================================

# vuln-code-snippet start hex_color_parse
def parse_hex_color(hex_str):
    # Standard CSS hex color parsing -- totally legitimate
    hex_str = hex_str.lstrip("#")
    rgb = bytes.fromhex(hex_str)
    return {"r": rgb[0], "g": rgb[1], "b": rgb[2]}  # vuln-code-snippet safe-line hex_color_parse
# vuln-code-snippet end hex_color_parse


# ============================================================================
# SAFE: getattr for documented plugin dispatch (legitimate)
# ============================================================================

# vuln-code-snippet start getattr_plugin_load
def load_plugin(plugin_name, registry):
    """Load a plugin by name from the registered plugin dictionary.

    Args:
        plugin_name: Name from ALLOWED_PLUGINS constant
        registry: Dict mapping names to plugin classes
    """
    if plugin_name not in registry:
        raise ValueError(f"Unknown plugin: {plugin_name}")
    plugin_class = registry[plugin_name]
    return plugin_class()  # vuln-code-snippet safe-line getattr_plugin_load
# vuln-code-snippet end getattr_plugin_load
