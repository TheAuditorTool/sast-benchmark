<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_exif_strip_reencode
function fileupload043(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $src  = imagecreatefromjpeg($file['tmp_name']);
    if ($src === false) {
        return BenchmarkResponse::badRequest('Invalid JPEG');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($src, $dest, 85); // vuln-code-snippet safe-line php_upload_exif_strip_reencode
    imagedestroy($src);
    return BenchmarkResponse::ok('Re-encoded without EXIF');
}
// vuln-code-snippet end php_upload_exif_strip_reencode
