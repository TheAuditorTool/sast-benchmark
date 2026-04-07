<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_csrf_upload_no_token
function csrf028(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['upload'] ?? null;
    if (!$file || $file['error'] !== UPLOAD_ERR_OK) {
        return BenchmarkResponse::badRequest('no file');
    }
    $dest = '/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_csrf_upload_no_token
    return BenchmarkResponse::ok('uploaded: ' . $dest);
}
// vuln-code-snippet end php_csrf_upload_no_token
