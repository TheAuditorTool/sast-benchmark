<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00017(BenchmarkRequest $req): BenchmarkResponse {
    $existing = ['name' => '', 'email' => '', 'role' => 'user'];
    $merged = array_replace($existing, $_POST);
    $model = new stdClass();
    foreach ($merged as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('replaced');
}
