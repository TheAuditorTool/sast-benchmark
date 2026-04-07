<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_compact_post_keys
function massassign025(BenchmarkRequest $req): BenchmarkResponse {
    extract($_POST);
    $data = compact(...array_keys($_POST));
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_compact_post_keys
    }
    return BenchmarkResponse::ok('compacted');
}
// vuln-code-snippet end php_massassign_compact_post_keys
