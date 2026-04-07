<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_force_attachment
function fileupload045(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.bin';
    move_uploaded_file($file['tmp_name'], $dest);
    header('Content-Disposition: attachment; filename="safe.pdf"'); // vuln-code-snippet safe-line php_upload_force_attachment
    header('Content-Type: application/octet-stream');
    readfile($dest);
    return BenchmarkResponse::ok('');
}
// vuln-code-snippet end php_upload_force_attachment
