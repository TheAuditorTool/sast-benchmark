<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_igbinary_post
function deserial024(BenchmarkRequest $req): BenchmarkResponse {
    $raw = $req->post('data');
    $obj = igbinary_unserialize(base64_decode($raw)); // vuln-code-snippet vuln-line php_deser_igbinary_post
    return BenchmarkResponse::ok('deserialized');
}
// vuln-code-snippet end php_deser_igbinary_post
