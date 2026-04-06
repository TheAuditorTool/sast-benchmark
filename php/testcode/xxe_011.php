<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_blind_dtd
function xxe011(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NONET); // vuln-code-snippet vuln-line php_xxe_blind_dtd
    $value = $doc->getElementsByTagName('data')->item(0)->textContent;
    return BenchmarkResponse::ok($value);
}
// vuln-code-snippet end php_xxe_blind_dtd
