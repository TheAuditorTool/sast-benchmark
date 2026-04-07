<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_dimensions_check
function fileupload039(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $info = getimagesize($file['tmp_name']);
    if ($info === false || $info[0] <= 0 || $info[1] <= 0) { // vuln-code-snippet safe-line php_upload_dimensions_check
        return BenchmarkResponse::badRequest('Invalid image dimensions');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.img';
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Image uploaded');
}
// vuln-code-snippet end php_upload_dimensions_check
