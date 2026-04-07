<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_ext_allowlist_strict
function fileupload036(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $ext  = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    if (!in_array($ext, ['jpg', 'png', 'gif'], true)) { // vuln-code-snippet safe-line php_upload_ext_allowlist_strict
        return BenchmarkResponse::badRequest('Extension not allowed');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_ext_allowlist_strict
