<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00202(BenchmarkRequest $req): BenchmarkResponse {
    $reader = new XMLReader();
    $reader->open('/var/app/uploads/' . basename($req->param('file')));
    $reader->setParserProperty(XMLReader::SUBST_ENTITIES, false);
    return BenchmarkResponse::ok('configured');
}
