/**
 * Visual Deception Test Cases (JavaScript)
 * Tests SAST tool ability to detect Trojan Source bidi overrides and homoglyphs.
 * CVE-2021-42574, CVE-2021-42694
 */

// ============================================================================
// VULNERABLE: Bidi override hides code execution in comment
// CVE-2021-42574 pattern
// ============================================================================

// vuln-code-snippet start bidi_comment_hide
function checkAccess(user) {
    // The line below contains U+202E (RLO) making code appear as comment
    // What looks like: /* if (isAdmin(user)) */ return true;
    // What executes:   return true;  (the "comment" is visually fake)
    /* \u202E } ;eurter nruter \u202C if (isAdmin(user)) { */
    return true;  // vuln-code-snippet vuln-line bidi_comment_hide
}
// vuln-code-snippet end bidi_comment_hide


// ============================================================================
// VULNERABLE: Homoglyph variable creates shadow bypass
// CVE-2021-42694 pattern
// ============================================================================

// vuln-code-snippet start homoglyph_variable
function validateCredential(input) {
    // "p\u0430ssword" uses Cyrillic 'a' (U+0430) instead of Latin 'a' (U+0061)
    // SAST sees two different variables; human sees one
    const password = getStoredHash();      // Latin 'a' - the real credential
    const p\u0430ssword = input;           // Cyrillic 'a' - shadow variable
    // This comparison always uses the attacker-controlled shadow variable
    if (p\u0430ssword === password) {      // vuln-code-snippet vuln-line homoglyph_variable
        grantAccess();
    }
}
// vuln-code-snippet end homoglyph_variable


// ============================================================================
// SAFE: Legitimate Arabic/Hebrew comment with natural bidi
// ============================================================================

// vuln-code-snippet start rtl_language_comment
function formatPrice(amount, currency) {
    // This function formats prices for RTL locales
    // Arabic comment with natural bidi: \u0633\u0639\u0631 (price)
    const formatted = new Intl.NumberFormat('ar-SA', {
        style: 'currency',
        currency: currency,
    }).format(amount);
    return formatted;  // vuln-code-snippet safe-line rtl_language_comment
}
// vuln-code-snippet end rtl_language_comment


// ============================================================================
// SAFE: Bidi characters in user-facing display string (legitimate)
// ============================================================================

// vuln-code-snippet start bidi_in_string_data
function formatBidiText(hebrewName, englishLabel) {
    // Using bidi marks in display strings is standard i18n practice
    const LRM = '\u200E';  // Left-to-Right Mark
    const RLM = '\u200F';  // Right-to-Left Mark
    return LRM + englishLabel + ': ' + RLM + hebrewName;  // vuln-code-snippet safe-line bidi_in_string_data
}
// vuln-code-snippet end bidi_in_string_data
