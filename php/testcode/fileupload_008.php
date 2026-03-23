<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_out_of_webroot
function fileupload008(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'png', 'gif', 'pdf'];
    if (!in_array($ext, $allowed, true)) {
        return BenchmarkResponse::badRequest('invalid extension');
    }
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext;
    $dest = '/var/uploads/' . $safeName; // vuln-code-snippet safe-line php_upload_out_of_webroot
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("stored as $safeName");
}
// vuln-code-snippet end php_upload_out_of_webroot
