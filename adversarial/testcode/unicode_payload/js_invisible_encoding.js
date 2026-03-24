/**
 * Unicode Payload Test Cases -- Invisible Character Encoding
 * Tests SAST tool's ability to detect invisible Unicode in string literals.
 *
 * NOTE: Some strings contain ACTUAL invisible Unicode characters.
 * They appear empty in editors but contain encoded data.
 * This file is a BENCHMARK TEST CASE for security tool evaluation.
 */

// ============================================================================
// VULNERABLE: Variation Selectors encoding payload in eval()
// ============================================================================

// vuln-code-snippet start vs_eval_payload
function decodeVariationSelectors(encoded) {
    // Glassworm-style decoder: maps Variation Selectors to byte values
    // VS1-VS16 (U+FE00-FE0F) map to 0-15
    // VS17-VS256 (U+E0100-E01EF) map to 16-255
    const s = v => [...v].map(w => (
        w = w.codePointAt(0),
        w >= 0xFE00 && w <= 0xFE0F ? w - 0xFE00 :
        w >= 0xE0100 && w <= 0xE01EF ? w - 0xE0100 + 16 : null
    )).filter(n => n !== null);
    // The backtick string contains Variation Selectors invisible to editors
    eval(Buffer.from(s(`\uFE06\uFE0F\uFE0E\uFE03\uFE0F\uFE0C\uFE05`)).toString('utf-8'));  // vuln-code-snippet vuln-line vs_eval_payload
}
// vuln-code-snippet end vs_eval_payload


// ============================================================================
// VULNERABLE: Zero-Width Characters encoding binary data
// ============================================================================

// vuln-code-snippet start zwc_binary_encode
function decodeZeroWidth(text) {
    // Zero-width binary encoding:
    //   U+200C (ZWNJ) = bit 0
    //   U+200D (ZWJ)  = bit 1
    //   U+200B (ZWS)  = byte separator
    const bits = [];
    let currentByte = '';
    for (const ch of text) {
        const cp = ch.codePointAt(0);
        if (cp === 0x200C) currentByte += '0';
        else if (cp === 0x200D) currentByte += '1';
        else if (cp === 0x200B && currentByte.length === 8) {
            bits.push(parseInt(currentByte, 2));
            currentByte = '';
        }
    }
    if (currentByte.length === 8) bits.push(parseInt(currentByte, 2));
    const decoded = String.fromCharCode(...bits);
    eval(decoded);  // vuln-code-snippet vuln-line zwc_binary_encode
}
// Encoded payload using zero-width characters
decodeZeroWidth('\u200D\u200D\u200C\u200C\u200C\u200D\u200C\u200D\u200B\u200D\u200D\u200C\u200D\u200C\u200D\u200C\u200C\u200B');
// vuln-code-snippet end zwc_binary_encode


// ============================================================================
// VULNERABLE: Tag Block characters hiding instructions
// ============================================================================

// vuln-code-snippet start tag_block_smuggle
function processTagBlock(input) {
    // Unicode Tag Block (U+E0020-E007F) mirrors ASCII but renders invisible
    // Used for LLM prompt injection and payload smuggling
    const decoded = [...input].map(ch => {
        const cp = ch.codePointAt(0);
        if (cp >= 0xE0020 && cp <= 0xE007F) return String.fromCharCode(cp - 0xE0000);
        return null;
    }).filter(Boolean).join('');
    eval(decoded);  // vuln-code-snippet vuln-line tag_block_smuggle
}
processTagBlock('\u{E0061}\u{E006C}\u{E0065}\u{E0072}\u{E0074}\u{E0028}\u{E0031}\u{E0029}');
// vuln-code-snippet end tag_block_smuggle


// ============================================================================
// VULNERABLE: BOM character mass-padding in eval argument
// ============================================================================

// vuln-code-snippet start bom_injection
function bomPaddedExec(code) {
    // 30 BOM characters (U+FEFF) padding before actual payload
    // String appears to be just the code but has massive invisible prefix
    const padded = '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF'
                 + '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF'
                 + '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF'
                 + code;
    eval(padded);  // vuln-code-snippet vuln-line bom_injection
}
// vuln-code-snippet end bom_injection


// ============================================================================
// SAFE: Normal emoji with ZWJ sequences (legitimate i18n)
// ============================================================================

// vuln-code-snippet start vs_emoji_legitimate
function renderFamilyEmoji() {
    // Family emoji uses ZWJ (U+200D) to join person emojis
    // This is standard Unicode emoji composition, NOT an attack
    const family = '\u{1F468}\u200D\u{1F469}\u200D\u{1F467}\u200D\u{1F466}';
    document.getElementById('output').textContent = family;  // vuln-code-snippet safe-line vs_emoji_legitimate
}
// vuln-code-snippet end vs_emoji_legitimate


// ============================================================================
// SAFE: Single BOM at file start (standard usage)
// ============================================================================

// vuln-code-snippet start bom_single_standard
function readBOMFile(filePath) {
    const fs = require('fs');
    const content = fs.readFileSync(filePath, 'utf-8');
    // Standard BOM stripping: single BOM at file start is normal UTF-8 practice
    const cleaned = content.charCodeAt(0) === 0xFEFF ? content.slice(1) : content;  // vuln-code-snippet safe-line bom_single_standard
    return JSON.parse(cleaned);
}
// vuln-code-snippet end bom_single_standard


// ============================================================================
// SAFE: Actually empty eval (no hidden chars)
// ============================================================================

// vuln-code-snippet start empty_eval_literal
function noopEval() {
    // This eval truly has an empty string -- zero invisible characters
    eval('');  // vuln-code-snippet safe-line empty_eval_literal
}
// vuln-code-snippet end empty_eval_literal


// ============================================================================
// SAFE: ZWJ in CSS font ligature context (legitimate)
// ============================================================================

// vuln-code-snippet start zwc_css_ligature
function setCSSLigature(element) {
    // CSS ligature configuration -- no invisible chars in the string
    const ligatureStyle = 'font-variant-ligatures: common-ligatures';
    element.style.cssText = ligatureStyle;  // vuln-code-snippet safe-line zwc_css_ligature
}
// vuln-code-snippet end zwc_css_ligature


// ============================================================================
// SAFE: Variation Selector for CJK glyph selection (legitimate typography)
// ============================================================================

// vuln-code-snippet start vs_font_selector
function renderCJKGlyph(character) {
    // VS17 (U+E0100) selects a specific glyph variant for CJK Unified Ideographs
    // This is a legitimate Unicode feature for East Asian typography
    const withVariant = character + '\u{E0100}';
    document.getElementById('cjk-display').textContent = withVariant;  // vuln-code-snippet safe-line vs_font_selector
}
// vuln-code-snippet end vs_font_selector
