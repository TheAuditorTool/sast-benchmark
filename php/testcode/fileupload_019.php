<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_zip_to_webroot
function fileupload019(BenchmarkRequest $req): BenchmarkResponse {
    $file = $_FILES['f'];
    $zip  = new ZipArchive();
    if ($zip->open($file['tmp_name']) !== true) {
        return BenchmarkResponse::error('Bad zip');
    }
    $zip->extractTo('/var/www/html/'); // vuln-code-snippet vuln-line php_upload_zip_to_webroot
    $zip->close();
    return BenchmarkResponse::ok('Extracted');
}
// vuln-code-snippet end php_upload_zip_to_webroot
