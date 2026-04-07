<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_symfony_disable_entities
function xxe043(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NOENT | LIBXML_NONET | LIBXML_NOERROR); // vuln-code-snippet safe-line php_xxe_symfony_disable_entities
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_symfony_disable_entities
