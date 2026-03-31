"""
Charset Mapping Test Cases (Python)
Tests SAST tool ability to detect Windows Best-Fit / WorstFit encoding attacks.

Background: DEVCORE disclosed "WorstFit" at Black Hat Europe 2024 (published Jan 2025).
Windows Best-Fit mapping silently converts Unicode characters to ANSI approximations
when processes use legacy code pages. CVE-2024-4577 (PHP-CGI, CVSS 9.8) exploited
this with U+00AD (soft hyphen) mapping to dash, enabling argument injection.

Reference: https://devco.re/blog/2025/01/09/worstfit-unveiling-hidden-transformers-in-windows-ansi/
"""
import os
import subprocess
import urllib.parse


# ============================================================================
# VULNERABLE: Soft Hyphen (U+00AD) maps to dash on Windows CP1252/932/936/949
# CVE-2024-4577 pattern -- argument injection via invisible character
# ============================================================================

# vuln-code-snippet start worstfit_soft_hyphen_arg
def run_tool_with_options(user_option):
    """Run a CLI tool with user-specified option string.

    On Windows with Japanese/Chinese/Korean code pages, the soft hyphen
    U+00AD silently maps to ASCII dash 0x2D. An attacker sends
    "\u00ADd allow_url_include=1" which DISPLAYS as "d allow_url_include=1"
    (no visible dash) but EXECUTES as "-d allow_url_include=1".
    """
    cmd = ["php-cgi", user_option]
    subprocess.run(cmd)  # vuln-code-snippet vuln-line worstfit_soft_hyphen_arg
# vuln-code-snippet end worstfit_soft_hyphen_arg


# ============================================================================
# VULNERABLE: Yen Sign (U+00A5) maps to backslash on Japanese Windows (CP932)
# Path traversal via invisible backslash
# ============================================================================

# vuln-code-snippet start worstfit_yen_pathtraver
def serve_user_file(base_dir, filename):
    """Serve a file from a base directory.

    On Japanese Windows (CP932), the yen sign U+00A5 silently converts
    to backslash 0x5C. An attacker sends "images\u00A5..\\..\\etc\\passwd"
    which displays as "images...etc\\passwd" (yen sign looks normal in
    Japanese locale) but resolves as "images\\..\\..\\etc\\passwd".
    """
    path = os.path.join(base_dir, filename)
    with open(path, "r") as f:
        return f.read()  # vuln-code-snippet vuln-line worstfit_yen_pathtraver
# vuln-code-snippet end worstfit_yen_pathtraver


# ============================================================================
# VULNERABLE: Won Sign (U+20A9) maps to backslash on Korean Windows (CP949)
# Path escape via Korean currency symbol
# ============================================================================

# vuln-code-snippet start worstfit_won_path_escape
def load_template(template_dir, user_template):
    """Load a template file by user-specified name.

    On Korean Windows (CP949), the won sign U+20A9 maps to backslash 0x5C.
    A filename containing won signs resolves to a path with backslashes,
    enabling directory traversal without any visible path separator character.
    """
    full_path = os.path.join(template_dir, user_template)
    with open(full_path, "r") as f:
        return f.read()  # vuln-code-snippet vuln-line worstfit_won_path_escape
# vuln-code-snippet end worstfit_won_path_escape


# ============================================================================
# VULNERABLE: Fullwidth Hyphen-Minus (U+FF0D) maps to dash on Windows
# Argument injection via visually distinct but semantically equivalent char
# ============================================================================

# vuln-code-snippet start worstfit_fullwidth_dash
def execute_command(program, user_args):
    """Execute a program with user-provided arguments.

    Fullwidth hyphen-minus U+FF0D maps to ASCII dash 0x2D under Best-Fit.
    User sends "\uFF0D-version" which displays as a fullwidth character
    (visually distinct from a dash in most editors) but executes as
    "--version" after Windows CP conversion.
    """
    cmd = [program] + user_args.split()
    result = subprocess.run(cmd, capture_output=True, text=True)
    return result.stdout  # vuln-code-snippet vuln-line worstfit_fullwidth_dash
# vuln-code-snippet end worstfit_fullwidth_dash


# ============================================================================
# VULNERABLE: Superscript digits (U+00B2, U+00B3) map to 2, 3 on Windows
# Port/index manipulation via invisible digit substitution
# ============================================================================

# vuln-code-snippet start worstfit_superscript_port
def proxy_request_to_backend(host, port_str, path):
    """Forward a request to a backend service at user-specified host:port.

    Superscript Two U+00B2 maps to ASCII '2' (0x32) and Superscript Three
    U+00B3 maps to ASCII '3' (0x33) under Windows Best-Fit mapping.

    The mapping occurs at the OS layer when the string is passed through
    Windows API functions (CreateProcessW, WideCharToMultiByte). A URL like
    "http://backend:80\u00B2\u00B3/admin" passes Python-level validation
    (port_str contains no ASCII digits matching a blocklist) but when the
    OS resolves it, the superscript digits become "8023", routing the
    request to an unintended internal port.
    """
    url = f"http://{host}:{port_str}/{path}"
    # The Best-Fit mapping happens when this URL is resolved by the OS
    # networking stack, not in Python's string handling
    response = subprocess.run(
        ["curl", "-s", url], capture_output=True, text=True
    )  # vuln-code-snippet vuln-line worstfit_superscript_port
    return response.stdout
# vuln-code-snippet end worstfit_superscript_port


# ============================================================================
# SAFE: Legitimate NFKC normalization for search indexing
# ============================================================================

# vuln-code-snippet start nfkc_normalize_search
def normalize_search_query(query):
    """Normalize Unicode search query to NFKC form for consistent indexing.

    NFKC normalization is a standard Unicode operation for search engines.
    It converts compatibility characters to their canonical equivalents
    (e.g., fullwidth 'A' to ASCII 'A') for consistent matching.
    This is a data transformation, not an execution context.
    """
    import unicodedata
    normalized = unicodedata.normalize("NFKC", query)
    return normalized.lower().strip()  # vuln-code-snippet safe-line nfkc_normalize_search
# vuln-code-snippet end nfkc_normalize_search


# ============================================================================
# SAFE: Unicode path with proper validation before filesystem use
# ============================================================================

# vuln-code-snippet start utf8_validated_path
def load_safe_file(base_dir, filename):
    """Load a file with proper path validation.

    The filename is resolved and checked against the base directory
    before any filesystem access. Even if charset mapping converts
    characters, the realpath check catches traversal attempts.
    """
    safe_path = os.path.realpath(os.path.join(base_dir, filename))
    if not safe_path.startswith(os.path.realpath(base_dir)):
        raise ValueError("Path traversal detected")
    with open(safe_path, "r") as f:
        return f.read()  # vuln-code-snippet safe-line utf8_validated_path
# vuln-code-snippet end utf8_validated_path


# ============================================================================
# SAFE: Fullwidth characters normalized for text search (data-only context)
# ============================================================================

# vuln-code-snippet start fullwidth_text_index
def build_search_index(documents):
    """Build a search index normalizing fullwidth characters.

    Fullwidth ASCII variants (U+FF01-U+FF5E) are normalized to their
    standard ASCII equivalents for consistent text matching. The output
    is a search index, never used in command execution or path operations.
    """
    import unicodedata
    index = {}
    for doc_id, text in documents:
        normalized = unicodedata.normalize("NFKC", text)
        for word in normalized.split():
            index.setdefault(word.lower(), []).append(doc_id)
    return index  # vuln-code-snippet safe-line fullwidth_text_index
# vuln-code-snippet end fullwidth_text_index
