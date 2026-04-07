<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_content_type_header_only
function fileupload027(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $mimeType = $file['type'];
    if (!in_array($mimeType, ['image/jpeg', 'image/png'], true)) {
        return BenchmarkResponse::badRequest('Invalid MIME type');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_content_type_header_only
    return BenchmarkResponse::ok('Image uploaded');
}
// vuln-code-snippet end php_upload_content_type_header_only
