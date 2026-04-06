<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_sax_parser
function xxe009(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    $data = [];
    xml_set_element_handler($parser, function($p, $name, $attrs) {}, function($p, $name) {});
    xml_set_character_data_handler($parser, function($p, $cdata) use (&$data) { $data[] = $cdata; });
    xml_parse($parser, $xml, true); // vuln-code-snippet vuln-line php_xxe_sax_parser
    xml_parser_free($parser);
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_xxe_sax_parser
