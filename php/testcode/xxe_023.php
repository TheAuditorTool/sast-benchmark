<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_billion_laughs
function xxe023(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_billion_laughs
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_billion_laughs
