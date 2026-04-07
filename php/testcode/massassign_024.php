<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_loop_assign
function massassign024(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    foreach ($_POST as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_loop_assign
    }
    return BenchmarkResponse::ok('saved');
}
// vuln-code-snippet end php_massassign_loop_assign
