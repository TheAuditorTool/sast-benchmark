<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_null_byte
function fileupload010(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $name = $file['name'];
    $ext = pathinfo($name, PATHINFO_EXTENSION);
    if ($ext !== 'jpg' && $ext !== 'png') {
        return BenchmarkResponse::badRequest('Only images allowed');
    }
    $dest = '/var/www/uploads/' . $name; // vuln-code-snippet vuln-line php_upload_null_byte
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_null_byte
