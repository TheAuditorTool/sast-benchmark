<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_hardcoded_xml_constant
function xxe038(BenchmarkRequest $req): BenchmarkResponse {
    $xml = '<root><item>value</item></root>';
    $data = simplexml_load_string($xml); // vuln-code-snippet safe-line php_xxe_hardcoded_xml_constant
    return BenchmarkResponse::ok((string)$data->item);
}
// vuln-code-snippet end php_xxe_hardcoded_xml_constant
