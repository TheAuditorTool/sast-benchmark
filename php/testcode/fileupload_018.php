<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_htaccess_override
function fileupload018(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $dest = '/var/www/uploads/' . $file['name'];
    move_uploaded_file($file['tmp_name'], $dest); // vuln-code-snippet vuln-line php_upload_htaccess_override
    return BenchmarkResponse::ok('Uploaded');
}
// vuln-code-snippet end php_upload_htaccess_override
