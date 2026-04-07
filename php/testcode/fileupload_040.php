<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_signed_url_expiry
function fileupload040(BenchmarkRequest $req): BenchmarkResponse {
    $file     = $_FILES['f'];
    $storedAs = bin2hex(random_bytes(16));
    move_uploaded_file($file['tmp_name'], '/var/uploads/' . $storedAs);
    $expires = time() + 3600;
    $sig     = hash_hmac('sha256', $storedAs . $expires, UPLOAD_SECRET);
    $url     = '/download/' . urlencode($storedAs) . "?exp={$expires}&sig={$sig}"; // vuln-code-snippet safe-line php_upload_signed_url_expiry
    return BenchmarkResponse::ok($url);
}
// vuln-code-snippet end php_upload_signed_url_expiry
