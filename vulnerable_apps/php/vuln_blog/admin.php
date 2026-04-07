<?php
// vuln_blog - Admin panel operations
require_once __DIR__ . '/config.php';

// vuln-code-snippet start vb_cmdi_image_resize
function resizeImageUnsafe(string $filename): void {
    exec('convert ' . $filename . ' -resize 200x200 output.jpg'); // vuln-code-snippet vuln-line vb_cmdi_image_resize
}
// vuln-code-snippet end vb_cmdi_image_resize

// vuln-code-snippet start vb_cmdi_image_safe
function resizeImageSafe(string $filename): void {
    exec('convert ' . escapeshellarg($filename) . ' -resize 200x200 output.jpg'); // vuln-code-snippet safe-line vb_cmdi_image_safe
}
// vuln-code-snippet end vb_cmdi_image_safe

// vuln-code-snippet start vb_codeinj_eval_config
function loadConfigUnsafe(): void {
    $configExpr = $_POST['config'];
    eval($configExpr); // vuln-code-snippet vuln-line vb_codeinj_eval_config
}
// vuln-code-snippet end vb_codeinj_eval_config

// vuln-code-snippet start vb_codeinj_safe
function loadConfigSafe(): array {
    $configJson = $_POST['config'];
    $config = json_decode($configJson, true); // vuln-code-snippet safe-line vb_codeinj_safe
    if ($config === null) {
        return [];
    }
    return $config;
}
// vuln-code-snippet end vb_codeinj_safe

// vuln-code-snippet start vb_deser_session
function loadPrefsUnsafe(): mixed {
    $prefs = $_COOKIE['prefs'];
    return unserialize($prefs); // vuln-code-snippet vuln-line vb_deser_session
}
// vuln-code-snippet end vb_deser_session

// vuln-code-snippet start vb_deser_json
function loadPrefsSafe(): mixed {
    $prefs = $_COOKIE['prefs'];
    return json_decode($prefs, true); // vuln-code-snippet safe-line vb_deser_json
}
// vuln-code-snippet end vb_deser_json

// vuln-code-snippet start vb_cookie_insecure
function setAdminCookieUnsafe(string $val): void {
    setcookie('admin', $val); // vuln-code-snippet vuln-line vb_cookie_insecure
}
// vuln-code-snippet end vb_cookie_insecure

// vuln-code-snippet start vb_cookie_secure
function setAdminCookieSafe(string $val): void {
    setcookie('admin', $val, ['secure' => true, 'httponly' => true, 'samesite' => 'Strict']); // vuln-code-snippet safe-line vb_cookie_secure
}
// vuln-code-snippet end vb_cookie_secure
