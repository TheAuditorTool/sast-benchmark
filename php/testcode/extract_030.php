<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_files_overwrite
function extract030(BenchmarkRequest $req): BenchmarkResponse {
    $uploadPath = '/var/www/uploads/';
    $dest = $uploadPath . 'file.dat';
    extract($_FILES); // vuln-code-snippet vuln-line php_extract_files_overwrite
    return BenchmarkResponse::ok("dest=$dest");
}
// vuln-code-snippet end php_extract_files_overwrite
