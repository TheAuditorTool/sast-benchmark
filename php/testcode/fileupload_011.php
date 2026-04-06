<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_mime_only
function fileupload011(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $finfo = finfo_open(FILEINFO_MIME_TYPE);
    $mime = finfo_file($finfo, $file['tmp_name']);
    finfo_close($finfo);
    if (!str_starts_with($mime, 'image/')) {
        return BenchmarkResponse::badRequest('Only images allowed');
    }
    $dest = '/var/www/uploads/' . $file['name']; // vuln-code-snippet vuln-line php_upload_mime_only
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_mime_only
