<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_simplexml
function xxe001(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = simplexml_load_string($xml); // vuln-code-snippet vuln-line php_xxe_simplexml
    if ($doc === false) {
        return BenchmarkResponse::badRequest('invalid xml');
    }
    return BenchmarkResponse::ok((string) $doc->name);
}
// vuln-code-snippet end php_xxe_simplexml
