<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_sax_no_handler
function xxe015(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    xml_parser_set_option($parser, XML_OPTION_SKIP_TAGSTART, 0);
    $data = [];
    xml_set_element_handler($parser, function($p, $name, $attrs) {}, function($p, $name) {});
    xml_set_character_data_handler($parser, function($p, $cdata) use (&$data) { $data[] = trim($cdata); });
    xml_set_external_entity_ref_handler($parser, function() { return false; }); // vuln-code-snippet safe-line php_xxe_sax_no_handler
    xml_parse($parser, $xml, true);
    xml_parser_free($parser);
    return BenchmarkResponse::json(array_filter($data));
}
// vuln-code-snippet end php_xxe_sax_no_handler
