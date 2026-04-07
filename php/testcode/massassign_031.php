<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_identity_map_post
function massassign031(BenchmarkRequest $req): BenchmarkResponse {
    $filtered = array_map(fn($v) => $v, $_POST);
    $model = new stdClass();
    foreach ($filtered as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_identity_map_post
    }
    return BenchmarkResponse::ok('mapped');
}
// vuln-code-snippet end php_massassign_identity_map_post
