<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_parsehuge_nonet
function xxe041(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $dom = new DOMDocument();
    $dom->loadXML($xml, LIBXML_PARSEHUGE | LIBXML_NONET); // vuln-code-snippet safe-line php_xxe_parsehuge_nonet
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_parsehuge_nonet
