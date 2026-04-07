<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_only_allowlist
function massassign032(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email', 'bio'];
    $data = array_intersect_key($_POST, array_flip($allowed)); // vuln-code-snippet safe-line php_massassign_only_allowlist
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('saved');
}
// vuln-code-snippet end php_massassign_only_allowlist
