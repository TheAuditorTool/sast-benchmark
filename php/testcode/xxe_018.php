<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_dom_load_xml
function xxe018(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_dom_load_xml
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_dom_load_xml
