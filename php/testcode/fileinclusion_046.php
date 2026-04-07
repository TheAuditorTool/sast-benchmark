<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_fi_cdn_signed_no_include
function fileinclusion046(BenchmarkRequest $req): BenchmarkResponse {
    $resource = $req->param('resource');
    $expires  = time() + 3600;
    $sig      = hash_hmac('sha256', $resource . $expires, CDN_SECRET);
    $url      = 'https://cdn.example.com/' . urlencode($resource) . "?exp={$expires}&sig={$sig}"; // vuln-code-snippet safe-line php_fi_cdn_signed_no_include
    return BenchmarkResponse::redirect($url);
}
// vuln-code-snippet end php_fi_cdn_signed_no_include
