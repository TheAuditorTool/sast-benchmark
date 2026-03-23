<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_no_validation
function fileupload001(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_no_validation
    return BenchmarkResponse::ok("uploaded to $dest");
}
// vuln-code-snippet end php_upload_no_validation
