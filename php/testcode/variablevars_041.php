<?php
require_once __DIR__ . '/shared.php';

/** @phpstan-immutable */
class StrictModel {
    public string $name = '';
}

// vuln-code-snippet start php_vv_no_dynamic_props_attribute
function variablevars041(BenchmarkRequest $req): BenchmarkResponse {
    $model = new StrictModel();
    $model->name = $req->param('name'); // vuln-code-snippet safe-line php_vv_no_dynamic_props_attribute
    return BenchmarkResponse::ok($model->name);
}
// vuln-code-snippet end php_vv_no_dynamic_props_attribute
