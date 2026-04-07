<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_uuid_filename
function fileupload034(BenchmarkRequest $req): BenchmarkResponse {
    $file    = $_FILES['f'];
    $ext     = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $safeName = bin2hex(random_bytes(16)) . '.' . $ext; // vuln-code-snippet safe-line php_upload_uuid_filename
    $dest = '/var/www/uploads/' . $safeName;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Uploaded as ' . $safeName);
}
// vuln-code-snippet end php_upload_uuid_filename
