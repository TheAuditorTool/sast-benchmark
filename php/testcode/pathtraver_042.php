<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_pt_url_validator_local_block
function pathtraver042(BenchmarkRequest $req): BenchmarkResponse {
    $url = $req->param('url');
    $parsed = parse_url($url);
    if (!in_array($parsed['scheme'] ?? '', ['https'], true)) {
        return BenchmarkResponse::badRequest('only https allowed');
    }
    $content = file_get_contents($url); // vuln-code-snippet safe-line php_pt_url_validator_local_block
    return BenchmarkResponse::ok($content);
}
// vuln-code-snippet end php_pt_url_validator_local_block
