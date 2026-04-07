<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_update_request_input
function massassign030(BenchmarkRequest $req): BenchmarkResponse {
    $input = array_merge($_GET, $_POST);
    $model = new stdClass();
    foreach ($input as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_update_request_input
    }
    return BenchmarkResponse::ok('updated');
}
// vuln-code-snippet end php_massassign_update_request_input
