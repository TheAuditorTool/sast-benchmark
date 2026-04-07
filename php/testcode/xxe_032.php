<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_disable_entity_loader
function xxe032(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(true); // vuln-code-snippet safe-line php_xxe_disable_entity_loader
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_disable_entity_loader
