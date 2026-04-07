<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_size_limit_enforced
function fileupload037(BenchmarkRequest $req): BenchmarkResponse {
    $file    = $_FILES['f'];
    $maxSize = 5 * 1024 * 1024;
    if ($file['size'] > $maxSize) { // vuln-code-snippet safe-line php_upload_size_limit_enforced
        return BenchmarkResponse::badRequest('File too large');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_size_limit_enforced
