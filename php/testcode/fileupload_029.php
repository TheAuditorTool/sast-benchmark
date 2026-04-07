<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_guessable_filename
function fileupload029(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dir  = '/var/www/uploads/';
    move_uploaded_file($file['tmp_name'], $dir . $file['name']); // vuln-code-snippet vuln-line php_upload_guessable_filename
    return BenchmarkResponse::ok('Uploaded as ' . $file['name']);
}
// vuln-code-snippet end php_upload_guessable_filename
