<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00134(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $reader = new XMLReader();
    $reader->XML($xml);
    $reader->setParserProperty(XMLReader::LOADDTD, false);
    $reader->setParserProperty(XMLReader::SUBST_ENTITIES, false);
    $data = [];
    while ($reader->read()) {
        if ($reader->nodeType === XMLReader::TEXT) {
            $data[] = $reader->value;
        }
    }
    $reader->close();
    return BenchmarkResponse::json($data);
}
