<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_gzip_decompressed_xml
function xxe027(BenchmarkRequest $req): BenchmarkResponse {
    $compressed = $req->bodyStr();
    $xml = gzinflate($compressed);
    $data = simplexml_load_string($xml); // vuln-code-snippet vuln-line php_xxe_gzip_decompressed_xml
    return BenchmarkResponse::ok((string)$data);
}
// vuln-code-snippet end php_xxe_gzip_decompressed_xml
