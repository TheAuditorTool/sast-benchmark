<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_explicit_update_field
function massassign040(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name']; // vuln-code-snippet safe-line php_massassign_explicit_update_field
    return BenchmarkResponse::ok('updated');
}
// vuln-code-snippet end php_massassign_explicit_update_field
