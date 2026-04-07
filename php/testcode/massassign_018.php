<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_merge_post_defaults
function massassign018(BenchmarkRequest $req): BenchmarkResponse {
    $data = array_merge($_POST, ['created_at' => date('Y-m-d')]);
    $model = new stdClass();
    foreach ($data as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_merge_post_defaults
    }
    return BenchmarkResponse::ok('created');
}
// vuln-code-snippet end php_massassign_merge_post_defaults
