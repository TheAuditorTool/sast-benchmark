<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_gzuncompress_chain
function deserial021(BenchmarkRequest $req): BenchmarkResponse {
    $payload = $req->post('payload');
    $decompressed = gzuncompress(base64_decode($payload));
    $obj = unserialize($decompressed); // vuln-code-snippet vuln-line php_deser_gzuncompress_chain
    return BenchmarkResponse::ok('processed');
}
// vuln-code-snippet end php_deser_gzuncompress_chain
