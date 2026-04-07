<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01079(BenchmarkRequest $req): BenchmarkResponse {
    $reader = new XMLReader();
    $reader->open('/var/app/data/config.xml');
    $reader->setParserProperty(XMLReader::VALIDATE, false);
    return BenchmarkResponse::ok('configured');
}
