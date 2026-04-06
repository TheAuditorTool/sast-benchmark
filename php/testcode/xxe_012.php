<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_parameter_entity
function xxe012(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->resolveExternals = true;
    $doc->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_parameter_entity
    $value = $doc->getElementsByTagName('data')->item(0)->textContent;
    return BenchmarkResponse::ok($value);
}
// vuln-code-snippet end php_xxe_parameter_entity
