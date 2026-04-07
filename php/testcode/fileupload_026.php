<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_phar_include_chain
function fileupload026(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_phar_include_chain
    return BenchmarkResponse::ok('PHAR uploaded');
}
// vuln-code-snippet end php_upload_phar_include_chain
