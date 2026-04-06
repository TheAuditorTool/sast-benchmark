<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_tempnam_ext
function fileupload016(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $tmpPath = tempnam('/var/www/uploads', 'up_');
    $safePath = $tmpPath . '.dat';
    rename($tmpPath, $safePath);
    move_uploaded_file($file['tmp_name'], $safePath); // vuln-code-snippet safe-line php_upload_tempnam_ext
    return BenchmarkResponse::ok('Uploaded to controlled path');
}
// vuln-code-snippet end php_upload_tempnam_ext
