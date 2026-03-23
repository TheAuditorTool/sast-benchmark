<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_yaml_parse
function deserial_yaml_parse(BenchmarkRequest $req): BenchmarkResponse {
    $yamlStr = $req->bodyStr();
    $data = yaml_parse($yamlStr); // vuln-code-snippet vuln-line php_deser_yaml_parse
    return BenchmarkResponse::json($data);
}
// vuln-code-snippet end php_deser_yaml_parse
