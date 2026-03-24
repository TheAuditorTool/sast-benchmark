/**
 * Dynamic Construction Test Cases (JavaScript)
 * Tests SAST tool ability to detect obfuscated code assembly.
 */

// ============================================================================
// VULNERABLE: String.fromCharCode reconstructs "eval"
// ============================================================================

// vuln-code-snippet start fromcharcode_eval
function stealthEval(payload) {
    // 101=e, 118=v, 97=a, 108=l
    const fn = String.fromCharCode(101, 118, 97, 108);
    window[fn](payload);  // vuln-code-snippet vuln-line fromcharcode_eval
}
// vuln-code-snippet end fromcharcode_eval


// ============================================================================
// VULNERABLE: Bracket notation with string concatenation
// ============================================================================

// vuln-code-snippet start bracket_notation_eval
function bracketEval(code) {
    const part1 = 'ev';
    const part2 = 'al';
    globalThis[part1 + part2](code);  // vuln-code-snippet vuln-line bracket_notation_eval
}
// vuln-code-snippet end bracket_notation_eval


// ============================================================================
// VULNERABLE: Multi-layer hex + rot13 + atob chain
// ============================================================================

// vuln-code-snippet start hex_rot13_chain
function rot13(str) {
    return str.replace(/[a-zA-Z]/g, c =>
        String.fromCharCode(c.charCodeAt(0) + (c.toLowerCase() < 'n' ? 13 : -13))
    );
}

function multiLayerDecode(encoded) {
    // Layer 1: hex decode
    const hexDecoded = encoded.match(/.{1,2}/g).map(b => String.fromCharCode(parseInt(b, 16))).join('');
    // Layer 2: rot13
    const rot13Decoded = rot13(hexDecoded);
    // Layer 3: base64 decode
    const payload = Buffer.from(rot13Decoded, 'base64').toString('utf-8');
    eval(payload);  // vuln-code-snippet vuln-line hex_rot13_chain
}
// vuln-code-snippet end hex_rot13_chain


// ============================================================================
// SAFE: String.fromCharCode for emoji logging (legitimate)
// ============================================================================

// vuln-code-snippet start charcode_logging
function logWithIcon(message, level) {
    // Using fromCharCode for log level icons (checkmark, warning, error)
    const icons = {
        info: String.fromCharCode(0x2139),     // information source
        warn: String.fromCharCode(0x26A0),     // warning sign
        error: String.fromCharCode(0x274C),    // cross mark
    };
    const icon = icons[level] || '';
    console.log(icon + ' ' + message);  // vuln-code-snippet safe-line charcode_logging
}
// vuln-code-snippet end charcode_logging


// ============================================================================
// SAFE: atob() for base64 image data URI (legitimate)
// ============================================================================

// vuln-code-snippet start base64_image_decode
function renderBase64Image(base64Data) {
    // Decoding base64 image data for canvas rendering — standard pattern
    const binaryString = atob(base64Data);
    const bytes = new Uint8Array(binaryString.length);
    for (let i = 0; i < binaryString.length; i++) {
        bytes[i] = binaryString.charCodeAt(i);
    }
    const blob = new Blob([bytes], { type: 'image/png' });  // vuln-code-snippet safe-line base64_image_decode
    return URL.createObjectURL(blob);
}
// vuln-code-snippet end base64_image_decode
