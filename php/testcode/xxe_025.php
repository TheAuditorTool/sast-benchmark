<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_external_dtd
function xxe025(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_DTDLOAD); // vuln-code-snippet vuln-line php_xxe_external_dtd
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_external_dtd
