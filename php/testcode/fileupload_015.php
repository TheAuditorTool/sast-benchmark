<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_upload_signed_url
function fileupload015(BenchmarkRequest $req): BenchmarkResponse {
    $key = $req->param('key');
    $sig = $req->param('sig');
    $expected = hash_hmac('sha256', $key, getenv('UPLOAD_SECRET'));
    if (!hash_equals($expected, $sig)) {
        return BenchmarkResponse::badRequest('Invalid upload signature');
    }
    $safePath = '/var/www/uploads/' . basename($key); // vuln-code-snippet safe-line php_upload_signed_url
    return BenchmarkResponse::json(['upload_url' => $safePath]);
}
// vuln-code-snippet end php_upload_signed_url
