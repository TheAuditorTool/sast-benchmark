<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_content_type_only
function fileupload003(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    if ($file['type'] !== 'image/jpeg' && $file['type'] !== 'image/png') { // vuln-code-snippet vuln-line php_upload_content_type_only
        return BenchmarkResponse::badRequest('only images allowed');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
// vuln-code-snippet end php_upload_content_type_only
