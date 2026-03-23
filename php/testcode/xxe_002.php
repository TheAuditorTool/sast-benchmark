<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_simplexml_disabled
function xxe002(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    libxml_disable_entity_loader(true); // vuln-code-snippet safe-line php_xxe_simplexml_disabled
    $doc = simplexml_load_string($xml, 'SimpleXMLElement', LIBXML_NONET);
    if ($doc === false) {
        return BenchmarkResponse::badRequest('invalid xml');
    }
    return BenchmarkResponse::ok((string) $doc->name);
}
// vuln-code-snippet end php_xxe_simplexml_disabled
