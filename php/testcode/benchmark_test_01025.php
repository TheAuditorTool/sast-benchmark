<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01025(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $cols = implode(',', array_keys($_POST));
    $vals = implode(',', array_fill(0, count($_POST), '?'));
    $db->prepare("INSERT INTO users ($cols) VALUES ($vals)")->execute(array_values($_POST));
    return BenchmarkResponse::ok('inserted');
}
