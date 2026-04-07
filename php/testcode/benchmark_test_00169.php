<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00169(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $elem = new SimpleXMLElement($xml, LIBXML_NONET | LIBXML_DTDLOAD | LIBXML_DTDATTR);
    return BenchmarkResponse::ok((string)$elem);
}
