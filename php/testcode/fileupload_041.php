<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_thumbnail_only_serve
function fileupload041(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $src  = imagecreatefromstring(file_get_contents($file['tmp_name']));
    if ($src === false) {
        return BenchmarkResponse::badRequest('Invalid image');
    }
    $thumb = imagescale($src, 200, 200); // vuln-code-snippet safe-line php_upload_thumbnail_only_serve
    $dest  = '/var/www/uploads/thumb_' . bin2hex(random_bytes(8)) . '.jpg';
    imagejpeg($thumb, $dest);
    imagedestroy($src);
    imagedestroy($thumb);
    return BenchmarkResponse::ok('Thumbnail created');
}
// vuln-code-snippet end php_upload_thumbnail_only_serve
