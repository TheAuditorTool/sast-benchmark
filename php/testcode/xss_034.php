<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_xss_json_encode_hex
function xss034(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('data');
    $safe = json_encode($input, JSON_HEX_TAG | JSON_HEX_AMP | JSON_HEX_APOS | JSON_HEX_QUOT); // vuln-code-snippet safe-line php_xss_json_encode_hex
    $html = "<script>var data = $safe</script>";
    return BenchmarkResponse::html($html);
}
// vuln-code-snippet end php_xss_json_encode_hex
