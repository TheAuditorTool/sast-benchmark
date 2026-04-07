<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00856(BenchmarkRequest $req): BenchmarkResponse {
    $allowed = ['name', 'email'];
    foreach (array_keys($_POST) as $k) {
        if (!in_array($k, $allowed, true)) {
            return BenchmarkResponse::badRequest('invalid field: ' . $k);
        }
    }
    $model = new stdClass();
    foreach ($_POST as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('validated');
}
