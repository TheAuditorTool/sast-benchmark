<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_outside_webroot
function fileupload033(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet safe-line php_upload_outside_webroot
    return BenchmarkResponse::ok('Stored outside webroot');
}
// vuln-code-snippet end php_upload_outside_webroot
