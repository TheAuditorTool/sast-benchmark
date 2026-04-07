<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_polyglot_jpeg_php
function fileupload028(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $tmp  = $file['tmp_name'];
    $magic = file_get_contents($tmp, false, null, 0, 3);
    if ($magic !== "\xFF\xD8\xFF") {
        return BenchmarkResponse::badRequest('Not a JPEG');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($tmp, $dest); // vuln-code-snippet vuln-line php_upload_polyglot_jpeg_php
    return BenchmarkResponse::ok('Image uploaded');
}
// vuln-code-snippet end php_upload_polyglot_jpeg_php
