<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_html_as_text
function fileupload023(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    if ($file['type'] !== 'text/plain') {
        return BenchmarkResponse::badRequest('Only text allowed');
    }
    $dest = '/var/www/uploads/' . basename($file['name']);
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_html_as_text
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_html_as_text
