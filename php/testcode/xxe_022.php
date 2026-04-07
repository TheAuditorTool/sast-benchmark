<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xml_parse_into_struct
function xxe022(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    xml_parse_into_struct($parser, $xml, $vals); // vuln-code-snippet vuln-line php_xxe_xml_parse_into_struct
    xml_parser_free($parser);
    return BenchmarkResponse::ok(json_encode($vals));
}
// vuln-code-snippet end php_xxe_xml_parse_into_struct
