<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_codeinj_json_eval
function codeinj018(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['user' => $req->param('name'), 'ts' => time()];
    $json = json_encode($data, JSON_HEX_TAG | JSON_HEX_AMP); // vuln-code-snippet safe-line php_codeinj_json_eval
    return BenchmarkResponse::ok($json);
}
// vuln-code-snippet end php_codeinj_json_eval
