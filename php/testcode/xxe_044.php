<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_trusted_s3_xml
function xxe044(BenchmarkRequest $req): BenchmarkResponse {
    $key = basename($req->param('key'));
    $xml = file_get_contents('s3://trusted-bucket/' . $key);
    libxml_disable_entity_loader(true); // vuln-code-snippet safe-line php_xxe_trusted_s3_xml
    $data = simplexml_load_string($xml);
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_trusted_s3_xml
