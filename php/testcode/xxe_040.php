<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xsd_dtd_disabled_config
function xxe040(BenchmarkRequest $req): BenchmarkResponse {
    libxml_disable_entity_loader(true); // vuln-code-snippet safe-line php_xxe_xsd_dtd_disabled_config
    $dom = new DOMDocument();
    $dom->loadXML($req->bodyStr(), LIBXML_NONET);
    $dom->schemaValidate('/etc/app/schema.xsd');
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_xsd_dtd_disabled_config
