<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01229(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $doc = simplexml_load_string($xml, 'SimpleXMLElement', LIBXML_NOENT);
    $title = (string)$doc->title;
    return BenchmarkResponse::ok($title);
}
