<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00019(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $reader = new XMLReader();
    $reader->xml($xml);
    $output = '';
    while ($reader->read()) {
        if ($reader->nodeType === XMLReader::TEXT) {
            $output .= $reader->value;
        }
    }
    return BenchmarkResponse::ok($output);
}
