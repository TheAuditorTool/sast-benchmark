"""
Visual Deception Test Cases (Python)
Tests SAST tool ability to detect homoglyph identifier attacks.
CVE-2021-42694
"""


# ============================================================================
# VULNERABLE: Bidi override reverses string comparison logic
# ============================================================================

# vuln-code-snippet start bidi_string_reverse
def check_token(token):
    # U+202E (RLO) in the string literal reverses display of comparison
    # Developer sees: if token == "admin_\u202Etoken_secret"
    # But the actual string value is different from what is displayed
    expected = "admin_\u202Etoken_secret"
    if token == expected:  # vuln-code-snippet vuln-line bidi_string_reverse
        return True
    return False
# vuln-code-snippet end bidi_string_reverse


# ============================================================================
# VULNERABLE: Cyrillic 'a' in function name creates shadow function
# ============================================================================

# vuln-code-snippet start homoglyph_function
def validate_input(data):
    """Legitimate validation function."""
    if not isinstance(data, str):
        raise ValueError("Input must be string")
    if len(data) > 1000:
        raise ValueError("Input too long")
    return data.strip()


# This function name uses Cyrillic 'a' (U+0430) instead of Latin 'a'
# It looks identical to "validate_input" in most fonts but is a different symbol
def vаlidate_input(data):  # vuln-code-snippet vuln-line homoglyph_function
    """Shadow function -- returns unsanitized input."""
    return data
# vuln-code-snippet end homoglyph_function


# ============================================================================
# VULNERABLE: Mixed-script import with confusable module name
# ============================================================================

# vuln-code-snippet start mixed_script_import
def load_crypto_module():
    # "crypt\u043E" uses Cyrillic 'o' (U+043E) instead of Latin 'o'
    # Attacker publishes a package named with the confusable character
    module_name = "crypt\u043E"
    mod = __import__(module_name)  # vuln-code-snippet vuln-line mixed_script_import
    return mod
# vuln-code-snippet end mixed_script_import


# ============================================================================
# SAFE: CJK variable names (legitimate i18n coding)
# ============================================================================

# vuln-code-snippet start unicode_identifier_i18n
def calculate_total():
    # Japanese variable names -- legitimate practice in some teams
    price = 1500
    quantity = 3
    total = price * quantity  # vuln-code-snippet safe-line unicode_identifier_i18n
    return total
# vuln-code-snippet end unicode_identifier_i18n
