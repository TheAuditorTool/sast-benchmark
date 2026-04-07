<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00149(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = $req->postData;
    $sets = [];
    foreach ($data as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute(array_merge(array_values($data), [$req->param('id')]));
    return BenchmarkResponse::json(['updated' => true]);
}
