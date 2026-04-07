<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_simplexml_multi_flags
function xxe036(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $elem = new SimpleXMLElement($xml, LIBXML_NONET | LIBXML_DTDLOAD | LIBXML_DTDATTR); // vuln-code-snippet safe-line php_xxe_simplexml_multi_flags
    return BenchmarkResponse::ok((string)$elem);
}
// vuln-code-snippet end php_xxe_simplexml_multi_flags
