<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_deser_yaml_object_tag
function deserial026(BenchmarkRequest $req): BenchmarkResponse {
    $body = $req->bodyStr();
    $data = yaml_parse($body); // vuln-code-snippet vuln-line php_deser_yaml_object_tag
    return BenchmarkResponse::json(is_array($data) ? $data : []);
}
// vuln-code-snippet end php_deser_yaml_object_tag
