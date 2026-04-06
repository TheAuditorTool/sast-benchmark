<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_dom_strip_doctype
function xxe014(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NOENT | LIBXML_DTDLOAD);
    $doctype = $doc->doctype;
    if ($doctype !== null) {
        $doc->removeChild($doctype); // vuln-code-snippet safe-line php_xxe_dom_strip_doctype
    }
    $value = $doc->getElementsByTagName('data')->item(0)->textContent ?? '';
    return BenchmarkResponse::ok($value);
}
// vuln-code-snippet end php_xxe_dom_strip_doctype
