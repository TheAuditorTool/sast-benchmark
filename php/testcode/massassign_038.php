<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_json_schema_validate
function massassign038(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email'];
    foreach (array_keys($_POST) as $k) {
        if (!in_array($k, $allowed, true)) { // vuln-code-snippet safe-line php_massassign_json_schema_validate
            return BenchmarkResponse::badRequest('invalid field: ' . $k);
        }
    }
    $model = new stdClass();
    foreach ($_POST as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('validated');
}
// vuln-code-snippet end php_massassign_json_schema_validate
