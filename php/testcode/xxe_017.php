<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_simplexml_no_disable
function xxe017(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml); // vuln-code-snippet vuln-line php_xxe_simplexml_no_disable
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_simplexml_no_disable
