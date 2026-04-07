<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_schema_only_validate
function xxe037(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NONET);
    $dom->schemaValidate('/etc/app/schema.xsd'); // vuln-code-snippet safe-line php_xxe_schema_only_validate
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_schema_only_validate
