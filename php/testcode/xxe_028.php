<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_multipart_upload_parse
function xxe028(BenchmarkRequest $req): BenchmarkResponse {
    $tmp = $_FILES['xml']['tmp_name'];
    $xml = file_get_contents($tmp);
    $dom = new DOMDocument();
    $dom->loadXML($xml); // vuln-code-snippet vuln-line php_xxe_multipart_upload_parse
    return BenchmarkResponse::ok($dom->saveXML());
}
// vuln-code-snippet end php_xxe_multipart_upload_parse
