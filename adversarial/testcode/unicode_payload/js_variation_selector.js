/**
 * Unicode Payload Test Cases — Invisible Character Encoding
 * Tests SAST tool's ability to detect invisible Unicode in string literals.
 *
 * IMPORTANT: Some strings below contain ACTUAL invisible Unicode characters.
 * They will appear empty in editors but contain encoded payloads.
 */

// ============================================================================
// VULNERABLE: Variation Selectors encoding payload in eval()
// ============================================================================

function decodeVariationSelectors(encoded) {
    // Glassworm-style decoder: maps Variation Selectors to byte values
    const s = v => [...v].map(w => (
        w = w.codePointAt(0),
        w >= 0xFE00 && w <= 0xFE0F ? w - 0xFE00 :
        w >= 0xE0100 && w <= 0xE01EF ? w - 0xE0100 + 16 : null
    )).filter(n => n !== null);
    // The backtick string below looks empty but contains 50+ Variation Selectors
    // encoding: console.log("pwned")
    eval(Buffer.from(s(`\uFE06\uFE0F\uFE0E\uFE03\uFE0F\uFE0C\uFE05\uFE02\uFE0C\uFE0F\uFE07\uFE02\uFE02\uFE08\uFE02\uFE02\uFE10\uFE17\uFE0E\uFE05\uFE04\uFE02\uFE02\uFE09`)).toString('utf-8'));
}


// ============================================================================
// VULNERABLE: Zero-Width Characters encoding binary data
// ============================================================================

function decodeZeroWidth(text) {
    // Zero-width binary encoding: U+200C = 0, U+200D = 1, U+200B = byte separator
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
    // The string below appears empty but contains ZWC-encoded payload
    eval(String.fromCharCode(...bits));
}
decodeZeroWidth('\u200D\u200D\u200C\u200C\u200C\u200D\u200C\u200D\u200B\u200D\u200D\u200C\u200D\u200C\u200D\u200C\u200C\u200B\u200D\u200D\u200C\u200C\u200C\u200D\u200C\u200D\u200B');


// ============================================================================
// VULNERABLE: Tag Block characters hiding payload
// ============================================================================

function processTagBlock(input) {
    // Unicode Tag Block (U+E0020-E007F) mirrors ASCII but is invisible
    const decoded = [...input].map(ch => {
        const cp = ch.codePointAt(0);
        if (cp >= 0xE0020 && cp <= 0xE007F) return String.fromCharCode(cp - 0xE0000);
        return null;
    }).filter(Boolean).join('');
    eval(decoded);
}
// Tag characters encoding "alert(1)" — invisible in all viewers
processTagBlock('\u{E0061}\u{E006C}\u{E0065}\u{E0072}\u{E0074}\u{E0028}\u{E0031}\u{E0029}');


// ============================================================================
// VULNERABLE: BOM character padding in eval argument
// ============================================================================

function bomPaddedEval(code) {
    // String looks like just "alert(1)" but has 30 BOM chars padding it
    const padded = '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF' +
                   '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF' +
                   '\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF\uFEFF' +
                   code;
    eval(padded);
}
bomPaddedEval('console.log("payload")');


// ============================================================================
// SAFE: Normal emoji with ZWJ sequences (legitimate i18n)
// ============================================================================

function renderFamilyEmoji() {
    // Family emoji uses ZWJ (U+200D) to join person emojis — legitimate use
    const family = '\u{1F468}\u200D\u{1F469}\u200D\u{1F467}\u200D\u{1F466}';
    document.getElementById('output').textContent = family;
}


// ============================================================================
// SAFE: Single BOM at file start (standard usage)
// ============================================================================

function readBOMFile(filePath) {
    const fs = require('fs');
    const content = fs.readFileSync(filePath, 'utf-8');
    // Standard BOM stripping — single BOM at start is normal
    const cleaned = content.charCodeAt(0) === 0xFEFF ? content.slice(1) : content;
    return JSON.parse(cleaned);
}


// ============================================================================
// SAFE: Actually empty eval (no hidden chars)
// ============================================================================

function noopEval() {
    // This eval truly has an empty string — no invisible characters
    eval('');
}


// ============================================================================
// SAFE: ZWJ in CSS font ligature (legitimate)
// ============================================================================

function setCSSLigature(element) {
    // CSS uses ZWJ for ligature control in font rendering
    const ligatureStyle = 'font-variant-ligatures: common-ligatures';
    element.style.cssText = ligatureStyle;
}


// ============================================================================
// SAFE: Variation Selector for CJK font glyph selection (legitimate)
// ============================================================================

function renderCJKGlyph(character) {
    // VS17 (U+E0100) selects specific glyph variant for CJK Unified Ideographs
    // This is a legitimate Unicode feature for East Asian typography
    const withVariant = character + '\u{E0100}';
    document.getElementById('cjk-display').textContent = withVariant;
}
