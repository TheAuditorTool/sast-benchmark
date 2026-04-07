<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_ext_bypass_variants
function fileupload022(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if ($ext === 'php') {
        return BenchmarkResponse::badRequest('PHP not allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_ext_bypass_variants
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_ext_bypass_variants
