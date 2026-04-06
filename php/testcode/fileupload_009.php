<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_htaccess
function fileupload009(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_htaccess
    return BenchmarkResponse::ok('File uploaded: ' . $file['name']);
}
// vuln-code-snippet end php_upload_htaccess
