<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_noent_flag_paradox
function xxe029(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $elem = new SimpleXMLElement($xml, LIBXML_NOENT); // vuln-code-snippet vuln-line php_xxe_noent_flag_paradox
    return BenchmarkResponse::ok((string)$elem);
}
// vuln-code-snippet end php_xxe_noent_flag_paradox
