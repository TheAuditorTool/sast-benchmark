<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00937(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    xml_set_external_entity_ref_handler($parser, fn() => false);
    xml_parse($parser, $xml);
    xml_parser_free($parser);
    return BenchmarkResponse::ok('parsed');
}
