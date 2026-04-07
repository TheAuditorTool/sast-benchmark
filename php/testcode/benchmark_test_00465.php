<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00465(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = new DOMDocument();
    $doc->loadXML($xml, LIBXML_NOENT | LIBXML_DTDLOAD);
    $doctype = $doc->doctype;
    if ($doctype !== null) {
        $doc->removeChild($doctype);
    }
    $value = $doc->getElementsByTagName('data')->item(0)->textContent ?? '';
    return BenchmarkResponse::ok($value);
}
