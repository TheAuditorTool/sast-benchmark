<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_fill_no_fillable
function massassign020(BenchmarkRequest $req): BenchmarkResponse {
    $data = $req->param('data');
    $decoded = json_decode($data, true);
    $model = new stdClass();
    foreach ($decoded as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_fill_no_fillable
    }
    return BenchmarkResponse::ok('filled');
}
// vuln-code-snippet end php_massassign_fill_no_fillable
