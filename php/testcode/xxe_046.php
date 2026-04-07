<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xxe_xml_to_json_boundary
function xxe046(BenchmarkRequest $req): BenchmarkResponse {
    $json = $req->bodyStr();
    $data = json_decode($json, true); // vuln-code-snippet safe-line php_xxe_xml_to_json_boundary
    return BenchmarkResponse::ok(json_encode($data));
}
// vuln-code-snippet end php_xxe_xml_to_json_boundary
