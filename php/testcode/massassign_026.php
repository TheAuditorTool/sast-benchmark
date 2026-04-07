<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_array_replace_post
function massassign026(BenchmarkRequest $req): BenchmarkResponse {
    $existing = ['name' => '', 'email' => '', 'role' => 'user'];
    $merged = array_replace($existing, $_POST);
    $model = new stdClass();
    foreach ($merged as $k => $v) {
        $model->$k = $v; // vuln-code-snippet vuln-line php_massassign_array_replace_post
    }
    return BenchmarkResponse::ok('replaced');
}
// vuln-code-snippet end php_massassign_array_replace_post
