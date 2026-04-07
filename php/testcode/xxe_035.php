<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_sax_reject_external
function xxe035(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $parser = xml_parser_create();
    xml_set_external_entity_ref_handler($parser, fn() => false); // vuln-code-snippet safe-line php_xxe_sax_reject_external
    xml_parse($parser, $xml);
    xml_parser_free($parser);
    return BenchmarkResponse::ok('parsed');
}
// vuln-code-snippet end php_xxe_sax_reject_external
