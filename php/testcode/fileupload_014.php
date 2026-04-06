<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_virus_scan
function fileupload014(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('upload');
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'png', 'gif', 'pdf'];
    if (!in_array($ext, $allowed, true)) { // vuln-code-snippet safe-line php_upload_virus_scan
        return BenchmarkResponse::badRequest('File type not allowed');
    }
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], '/var/www/uploads/' . $safeName);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
// vuln-code-snippet end php_upload_virus_scan
