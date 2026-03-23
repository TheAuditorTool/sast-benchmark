<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xmlreader
function xxe005(BenchmarkRequest $req): BenchmarkResponse {
    $xml = $req->bodyStr();
    $reader = new XMLReader();
    $reader->xml($xml); // vuln-code-snippet vuln-line php_xxe_xmlreader
    $output = '';
    while ($reader->read()) {
        if ($reader->nodeType === XMLReader::TEXT) {
            $output .= $reader->value;
        }
    }
    return BenchmarkResponse::ok($output);
}
// vuln-code-snippet end php_xxe_xmlreader
