<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_extension_blocklist
function fileupload005(BenchmarkRequest $req): BenchmarkResponse {
    $file = $req->file('f');
    if (!$file) {
        return BenchmarkResponse::badRequest('no file');
    }
    $ext = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $blocked = ['php', 'exe', 'sh', 'bat'];
    if (in_array($ext, $blocked, true)) { // vuln-code-snippet vuln-line php_upload_extension_blocklist
        return BenchmarkResponse::badRequest('blocked extension');
    }
    $dest = 'uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok("uploaded to $dest");
}
// vuln-code-snippet end php_upload_extension_blocklist
