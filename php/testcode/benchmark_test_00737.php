<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00737(BenchmarkRequest $req): BenchmarkResponse {
    $resource = $req->param('resource');
    $expires  = time() + 3600;
    $sig      = hash_hmac('sha256', $resource . $expires, CDN_SECRET);
    $url      = 'https://cdn.example.com/' . urlencode($resource) . "?exp={$expires}&sig={$sig}";
    return BenchmarkResponse::redirect($url);
}
