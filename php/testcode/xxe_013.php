<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xmlreader_no_dtd
function xxe013(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $reader = new XMLReader();
    $reader->XML($xml);
    $reader->setParserProperty(XMLReader::LOADDTD, false); // vuln-code-snippet safe-line php_xxe_xmlreader_no_dtd
    $reader->setParserProperty(XMLReader::SUBST_ENTITIES, false);
    $data = [];
    while ($reader->read()) {
        if ($reader->nodeType === XMLReader::TEXT) {
            $data[] = $reader->value;
        }
    }
    $reader->close();
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_xxe_xmlreader_no_dtd
