<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_attribute_cast_validate
function massassign043(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = (string) substr($_POST['name'] ?? '', 0, 255); // vuln-code-snippet safe-line php_massassign_attribute_cast_validate
    $model->age = (int) ($_POST['age'] ?? 0);
    return BenchmarkResponse::ok('cast');
}
// vuln-code-snippet end php_massassign_attribute_cast_validate
