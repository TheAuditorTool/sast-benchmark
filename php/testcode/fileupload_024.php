<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_path_traversal_name
function fileupload024(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $filename = $_POST['filename'];
    $dir      = '/var/www/uploads/';
    move_uploaded_file($file['tmp_name'], $dir . $filename); // vuln-code-snippet vuln-line php_upload_path_traversal_name
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_path_traversal_name
