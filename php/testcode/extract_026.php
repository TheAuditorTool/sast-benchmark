<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_extract_unserialize_chain
function extract026(BenchmarkRequest $req): BenchmarkResponse {
    $cookie = $req->cookie('state');
    $data = unserialize(base64_decode($cookie));
    extract($data); // vuln-code-snippet vuln-line php_extract_unserialize_chain
    return BenchmarkResponse::ok("step=$step");
}
// vuln-code-snippet end php_extract_unserialize_chain
