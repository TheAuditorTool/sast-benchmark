<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00682(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    xml_parse_into_struct($parser, $xml, $vals);
    xml_parser_free($parser);
    return BenchmarkResponse::ok(json_encode($vals));
}
