<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00634(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $cols = [];
    $vals = [];
    foreach ($data as $key => $val) {
        $cols[] = $key;
        $vals[] = $val;
    }
    $colStr = implode(', ', $cols);
    $placeholders = implode(', ', array_fill(0, count($vals), '?'));
    $pdo->prepare("INSERT INTO profiles ($colStr) VALUES ($placeholders)")->execute($vals);
    return BenchmarkResponse::ok('Profile created');
}
