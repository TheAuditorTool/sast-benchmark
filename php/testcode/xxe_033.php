<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_libxml_nonet_dtdattr
function xxe033(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_NONET | LIBXML_DTDATTR); // vuln-code-snippet safe-line php_xxe_libxml_nonet_dtdattr
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_libxml_nonet_dtdattr
