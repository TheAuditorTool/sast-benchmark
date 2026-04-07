<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xmlreader_no_validate
function xxe039(BenchmarkRequest $req): BenchmarkResponse {
    $reader = new XMLReader();
    $reader->open('/var/app/data/config.xml');
    $reader->setParserProperty(XMLReader::VALIDATE, false); // vuln-code-snippet safe-line php_xxe_xmlreader_no_validate
    return BenchmarkResponse::ok('configured');
}
// vuln-code-snippet end php_xxe_xmlreader_no_validate
