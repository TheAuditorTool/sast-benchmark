/**
 * Charset Mapping Test Cases (JavaScript)
 * Tests SAST tool ability to detect Windows Best-Fit / WorstFit encoding attacks.
 *
 * Includes both vulnerable patterns (invisible characters in command/path contexts)
 * and safe variants (same characters in data-only display contexts).
 */

const { execSync } = require('child_process');


// ============================================================================
// VULNERABLE: Soft Hyphen (U+00AD) in Node.js command argument
// Mirrors the Python worstfit_soft_hyphen_arg pattern in a JS context
// ============================================================================

// vuln-code-snippet start worstfit_node_soft_hyphen
function runToolWithUserFlag(userFlag) {
    // On Windows with CJK code pages, U+00AD (soft hyphen) maps to ASCII
    // dash 0x2D. An attacker sends "\u00ADversion" which displays as
    // "version" (soft hyphens are invisible in most renderers) but when
    // Windows passes this to the child process, Best-Fit mapping converts
    // it to "-version", injecting an argument flag.
    //
    // Input validation that blocks strings starting with "-" won't catch
    // this because the string starts with U+00AD, not ASCII 0x2D.
    const output = execSync(`mytool ${userFlag}`, { encoding: 'utf-8' });  // vuln-code-snippet vuln-line worstfit_node_soft_hyphen
    return output;
}
// vuln-code-snippet end worstfit_node_soft_hyphen


// ============================================================================
// SAFE: Yen sign in currency display string (legitimate i18n)
// ============================================================================

// vuln-code-snippet start yen_currency_display
function formatJapanesePrice(amount) {
    // U+00A5 (yen sign) used in its intended context: currency display.
    // On Japanese Windows, this character maps to backslash under Best-Fit,
    // but here it is only used in a display string, never in a path or command.
    const yen = '\u00A5';
    const formatted = yen + amount.toLocaleString('ja-JP');
    document.getElementById('price-display').textContent = formatted;  // vuln-code-snippet safe-line yen_currency_display
}
// vuln-code-snippet end yen_currency_display


// ============================================================================
// SAFE: Won sign in Korean locale label (legitimate i18n)
// ============================================================================

// vuln-code-snippet start korean_won_label
function formatKoreanPrice(amount) {
    // U+20A9 (won sign) in its intended context: Korean currency formatting.
    // The character maps to backslash under Best-Fit on Korean Windows,
    // but here it is only used as a UI label, never touches the filesystem.
    const won = '\u20A9';
    const label = won + amount.toLocaleString('ko-KR');
    return { currency: 'KRW', display: label };  // vuln-code-snippet safe-line korean_won_label
}
// vuln-code-snippet end korean_won_label
