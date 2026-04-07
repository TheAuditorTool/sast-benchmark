<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_explicit_field_assign
function massassign034(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name'] ?? ''; // vuln-code-snippet safe-line php_massassign_explicit_field_assign
    $model->email = $_POST['email'] ?? '';
    return BenchmarkResponse::ok('assigned');
}
// vuln-code-snippet end php_massassign_explicit_field_assign
