<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_clamav_scan
function fileupload046(BenchmarkRequest $req): BenchmarkResponse {
    $file   = $_FILES['f'];
    $tmp    = $file['tmp_name'];
    $output = [];
    $code   = 0;
    exec('claamscan --no-summary ' . escapeshellarg($tmp), $output, $code);
    if ($code !== 0) { // vuln-code-snippet safe-line php_upload_clamav_scan
        return BenchmarkResponse::badRequest('File rejected by antivirus');
    }
    $dest = '/var/www/uploads/' . bin2hex(random_bytes(8)) . '.dat';
    move_uploaded_file($tmp, $dest);
    return BenchmarkResponse::ok('Clean file uploaded');
}
// vuln-code-snippet end php_upload_clamav_scan
