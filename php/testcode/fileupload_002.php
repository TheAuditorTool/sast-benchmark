<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_uuid_rename
function fileupload002(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $allowed = ['jpg', 'png', 'gif'];
    if (!in_array($ext, $allowed, true)) { // vuln-code-snippet safe-line php_upload_uuid_rename
        return BenchmarkResponse::badRequest('invalid extension');
    }
    $dest = 'uploads/' . bin2hex(random_bytes(16)) . '.' . $ext;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
// vuln-code-snippet end php_upload_uuid_rename
