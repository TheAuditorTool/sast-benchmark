<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_yaml_safe
function deserial_yaml_safe(BenchmarkRequest $req): BenchmarkResponse {
    $yamlStr = $req->bodyStr();
    $ndocs = 0;
    $data = yaml_parse($yamlStr, 0, $ndocs, []); // vuln-code-snippet safe-line php_deser_yaml_safe
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_yaml_safe
