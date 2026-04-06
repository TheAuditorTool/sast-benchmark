<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_igbinary
function deserial013(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->post('payload');
    $obj = igbinary_unserialize(base64_decode($data)); // vuln-code-snippet vuln-line php_deser_igbinary
    return BenchmarkResponse::json(['type' => get_class($obj)]);
}
// vuln-code-snippet end php_deser_igbinary
