<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_double_extension
function fileupload007(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $imageExts = ['jpg', 'png', 'gif', 'jpeg'];
    if (!in_array($ext, $imageExts, true)) { // vuln-code-snippet vuln-line php_upload_double_extension
        return BenchmarkResponse::badRequest('images only');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
// vuln-code-snippet end php_upload_double_extension
