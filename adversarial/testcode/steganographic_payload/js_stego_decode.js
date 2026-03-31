/**
 * Steganographic Payload Test Cases (JavaScript)
 * Tests SAST tool ability to detect payloads hidden in non-code files.
 *
 * Based on: buildrunner-dev npm package (February 2026) which hid .NET
 * malware in PNG pixel RGB values. First 2 pixels encoded payload size
 * as uint32; remaining pixels carried 3 bytes per pixel in R/G/B channels.
 *
 * Reference: https://www.veracode.com/blog/malicious-npm-package-hiding-in-plain-pixels/
 */

const fs = require('fs');


// ============================================================================
// VULNERABLE: Extract payload from PNG pixel RGB values and eval
// Pattern from buildrunner-dev npm package (Feb 2026)
// ============================================================================

// vuln-code-snippet start png_pixel_decode_eval
function loadPluginFromImage(imagePath) {
    // Read PNG file and extract raw pixel data
    // buildrunner-dev pattern: first 2 pixels = payload size (uint32),
    // remaining pixels carry 3 bytes per pixel in R, G, B channels
    const { createCanvas, loadImage } = require('canvas');
    loadImage(imagePath).then(img => {
        const canvas = createCanvas(img.width, img.height);
        const ctx = canvas.getContext('2d');
        ctx.drawImage(img, 0, 0);
        const pixels = ctx.getImageData(0, 0, img.width, img.height).data;

        // First pixel RGB + second pixel R = 4 bytes encoding payload length as uint32
        const size = (pixels[0] << 24) | (pixels[1] << 16) | (pixels[2] << 8) | pixels[4];

        // Extract payload bytes from remaining pixel RGB channels (skip alpha)
        // Each pixel contributes 3 payload bytes (R, G, B); stride 4 skips alpha
        const payload = [];
        for (let i = 8; i < 8 + Math.ceil(size / 3) * 4; i += 4) {
            payload.push(pixels[i], pixels[i + 1], pixels[i + 2]);
        }
        const code = Buffer.from(payload.slice(0, size)).toString('utf-8');
        eval(code);  // vuln-code-snippet vuln-line png_pixel_decode_eval
    });
}
// vuln-code-snippet end png_pixel_decode_eval


// ============================================================================
// VULNERABLE: Extract payload from font file binary offset and eval
// ============================================================================

// vuln-code-snippet start font_binary_payload
function loadFontPlugin(fontPath) {
    // Read font file and extract bytes from a specific offset range
    // Attacker hides executable code in unused font table regions
    const data = fs.readFileSync(fontPath);

    // "name" table at known offset contains hidden payload
    const nameTableOffset = data.readUInt32BE(12);
    const nameTableLength = data.readUInt32BE(16);
    const payload = data.slice(nameTableOffset, nameTableOffset + nameTableLength);
    const code = payload.toString('utf-8');
    eval(code);  // vuln-code-snippet vuln-line font_binary_payload
}
// vuln-code-snippet end font_binary_payload


// ============================================================================
// SAFE: Read PNG pixels for canvas rendering (legitimate image processing)
// ============================================================================

// vuln-code-snippet start image_canvas_render
function resizeImage(imagePath, targetWidth, targetHeight) {
    // Standard image resizing -- reads pixel data for rendering, not execution
    const { createCanvas, loadImage } = require('canvas');
    return loadImage(imagePath).then(img => {
        const canvas = createCanvas(targetWidth, targetHeight);
        const ctx = canvas.getContext('2d');
        ctx.drawImage(img, 0, 0, targetWidth, targetHeight);
        return canvas.toBuffer('image/png');  // vuln-code-snippet safe-line image_canvas_render
    });
}
// vuln-code-snippet end image_canvas_render


// ============================================================================
// SAFE: Read font file for glyph metrics and kerning (legitimate typography)
// ============================================================================

// vuln-code-snippet start font_metrics_load
function loadFontMetrics(fontPath) {
    // Read font file for glyph width and kerning data -- standard typography
    const opentype = require('opentype.js');
    const font = opentype.loadSync(fontPath);
    const metrics = {};
    for (let i = 32; i < 127; i++) {
        const glyph = font.charToGlyph(String.fromCharCode(i));
        metrics[String.fromCharCode(i)] = {
            width: glyph.advanceWidth,
            name: glyph.name,
        };
    }
    return metrics;  // vuln-code-snippet safe-line font_metrics_load
}
// vuln-code-snippet end font_metrics_load
