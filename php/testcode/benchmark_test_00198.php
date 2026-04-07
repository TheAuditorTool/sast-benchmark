<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00198(BenchmarkRequest $req): BenchmarkResponse {
    $model = new stdClass();
    $model->name = $_POST['name'] ?? '';
    if ($_SESSION['is_admin']) {
        $model->role = $_POST['role'];
    }
    return BenchmarkResponse::ok('gated');
}
