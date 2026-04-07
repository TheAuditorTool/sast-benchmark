<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_getimagesize_only
function fileupload021(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $info = getimagesize($file['tmp_name']);
    if ($info === false) {
        return BenchmarkResponse::badRequest('Not an image');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_getimagesize_only
    return BenchmarkResponse::ok('Image uploaded');
}
// vuln-code-snippet end php_upload_getimagesize_only
