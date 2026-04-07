<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00015(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    $data = [];
    xml_set_element_handler($parser, function($p, $name, $attrs) {}, function($p, $name) {});
    xml_set_character_data_handler($parser, function($p, $cdata) use (&$data) { $data[] = $cdata; });
    xml_parse($parser, $xml, true);
    xml_parser_free($parser);
    return BenchmarkResponse::json($data);
}
