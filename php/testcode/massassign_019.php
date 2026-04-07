<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_set_raw_attributes
function massassign019(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $attrs = $_POST;
    foreach ($attrs as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_set_raw_attributes
    }
    return BenchmarkResponse::ok('updated');
}
// vuln-code-snippet end php_massassign_set_raw_attributes
