<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00910(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $elem = new SimpleXMLElement($xml, LIBXML_NOENT);
    return BenchmarkResponse::ok((string)$elem);
}
