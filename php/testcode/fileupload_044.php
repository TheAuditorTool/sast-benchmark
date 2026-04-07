<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_hash_filename
function fileupload044(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $ext      = strtolower(pathinfo($file['name'], PATHINFO_EXTENSION));
    $safeName = hash_file('sha256', $file['tmp_name']) . '.' . $ext; // vuln-code-snippet safe-line php_upload_hash_filename
    $dest     = '/var/www/uploads/' . $safeName;
    move_uploaded_file($file['tmp_name'], $dest);
    return BenchmarkResponse::ok('Stored as ' . $safeName);
}
// vuln-code-snippet end php_upload_hash_filename
