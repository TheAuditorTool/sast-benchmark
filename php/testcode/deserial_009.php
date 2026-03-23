<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_base64_chain
function deserial_base64_chain(BenchmarkRequest $req): BenchmarkResponse {
    $encoded = $req->param('obj');
    $decoded = base64_decode($encoded);
    $obj = unserialize($decoded); // vuln-code-snippet vuln-line php_deser_base64_chain
    return BenchmarkResponse::json(['result' => $obj]);
}
// vuln-code-snippet end php_deser_base64_chain
