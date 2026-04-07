<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_subst_entities_false
function xxe045(BenchmarkRequest $req): BenchmarkResponse {
    $reader = new XMLReader();
    $reader->open('/var/app/uploads/' . basename($req->param('file')));
    $reader->setParserProperty(XMLReader::SUBST_ENTITIES, false); // vuln-code-snippet safe-line php_xxe_subst_entities_false
    return BenchmarkResponse::ok('configured');
}
// vuln-code-snippet end php_xxe_subst_entities_false
