<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_gd_reencode
function fileupload032(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $img  = imagecreatefromjpeg($file['tmp_name']);
    if ($img === false) {
        return BenchmarkResponse::badRequest('Invalid image');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($img, $dest); // vuln-code-snippet safe-line php_upload_gd_reencode
    imagedestroy($img);
    return BenchmarkResponse::ok('Image stored');
}
// vuln-code-snippet end php_upload_gd_reencode
