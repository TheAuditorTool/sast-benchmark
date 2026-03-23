<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_domdocument_safe
function xxe004(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NOENT | LIBXML_NONET); // vuln-code-snippet safe-line php_xxe_domdocument_safe
    $name = $doc->getElementsByTagName('name')->item(0)->textContent ?? '';
    return BenchmarkResponse::ok($name);
}
// vuln-code-snippet end php_xxe_domdocument_safe
