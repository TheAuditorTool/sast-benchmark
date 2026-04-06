<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_gd_reprocess
function fileupload013(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $img = imagecreatefromstring(file_get_contents($file['tmp_name']));
    if ($img === false) {
        return BenchmarkResponse::badRequest('Not a valid image');
    }
    $safeName = bin2hex(random_bytes(16)) . '.png';
    imagepng($img, '/var/www/uploads/' . $safeName); // vuln-code-snippet safe-line php_upload_gd_reprocess
    imagedestroy($img);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
// vuln-code-snippet end php_upload_gd_reprocess
