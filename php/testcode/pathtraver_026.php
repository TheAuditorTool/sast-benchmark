<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_upload_traversal
function pathtraver026(BenchmarkRequest $req): BenchmarkResponse {
    $baseDir = '/var/app/uploads/';
    $filename = $req->post('filename');
    move_uploaded_file($_FILES['file']['tmp_name'], $baseDir . $filename); // vuln-code-snippet vuln-line php_pt_upload_traversal
    return BenchmarkResponse::ok('uploaded');
}
// vuln-code-snippet end php_pt_upload_traversal
