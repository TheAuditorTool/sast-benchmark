<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_parameter_entity_exfil
function xxe024(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    libxml_use_internal_errors(true);
    $data = simplexml_load_string($xml); // vuln-code-snippet vuln-line php_xxe_parameter_entity_exfil
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_parameter_entity_exfil
