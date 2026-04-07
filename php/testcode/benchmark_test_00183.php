<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00183(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $reader = new XMLReader();
    $reader->xml($xml);
    $reader->setParserProperty(XMLReader::SUBST_ENTITIES, false);
    $output = '';
    while ($reader->read()) {
        if ($reader->nodeType === XMLReader::TEXT) {
            $output .= $reader->value;
        }
    }
    return BenchmarkResponse::ok($output);
}
