<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00340(BenchmarkRequest $req): BenchmarkResponse {
    $rules = ['name' => 'required|string', 'email' => 'required|email'];
    $validated = array_intersect_key($_POST, $rules);
    $model = new stdClass();
    foreach ($validated as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('validated');
}
