<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_domdocument
function xxe003(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_domdocument
    $name = $doc->getElementsByTagName('name')->item(0)->textContent;
    return BenchmarkResponse::ok($name);
}
// vuln-code-snippet end php_xxe_domdocument
