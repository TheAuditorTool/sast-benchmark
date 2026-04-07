<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_repository_field_map
function massassign039(BenchmarkRequest $req): BenchmarkResponse {
    $data = ['name' => $_POST['name'], 'email' => $_POST['email']]; // vuln-code-snippet safe-line php_massassign_repository_field_map
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('mapped');
}
// vuln-code-snippet end php_massassign_repository_field_map
