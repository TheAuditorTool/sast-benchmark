<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_explicit_reenable
function xxe019(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(false); // vuln-code-snippet vuln-line php_xxe_explicit_reenable
    $xml = $req->bodyStr();
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_explicit_reenable
