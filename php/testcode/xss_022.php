<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_content_type_json
function xss_content_type_json(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('q');

    $data = ['query' => $input, 'results' => []]; // vuln-code-snippet safe-line php_xss_content_type_json

    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_xss_content_type_json
