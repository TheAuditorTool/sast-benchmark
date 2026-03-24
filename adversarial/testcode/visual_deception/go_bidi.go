package visual_deception

// Visual Deception Test Cases (Go)
// Tests for Trojan Source bidi early-return attack (CVE-2021-42574)

// ============================================================================
// VULNERABLE: Bidi makes return look like it's inside if-block but isn't
// ============================================================================

// vuln-code-snippet start bidi_early_return
func CheckPermission(user string) bool {
	// The bidi override below makes "return true" visually appear
	// inside the if-block, but it actually executes unconditionally.
	// U+202A (LRE) + U+202B (RLE) manipulate visual ordering.
	if user == "admin" {
		// \u202A \u202B
	}
	return true // vuln-code-snippet vuln-line bidi_early_return
}

// vuln-code-snippet end bidi_early_return

// ============================================================================
// SAFE: Standard ASCII identifiers, no visual tricks
// ============================================================================

// vuln-code-snippet start ascii_only_identifiers
func ValidateToken(token string) bool {
	expectedToken := GetStoredToken()
	if token == "" {
		return false
	}
	return token == expectedToken // vuln-code-snippet safe-line ascii_only_identifiers
}

// vuln-code-snippet end ascii_only_identifiers
