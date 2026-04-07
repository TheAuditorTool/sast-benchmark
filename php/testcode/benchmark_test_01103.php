<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01103(BenchmarkRequest $req): BenchmarkResponse {
    $guarded = ['is_admin', 'role'];
    $data = array_diff_key($req->postData, array_flip($guarded));
    $pdo = getDbConnection();
    $sets = [];
    foreach ($data as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $stmt = $pdo->prepare($sql);
    $stmt->execute(array_merge(array_values($data), [$req->param('id')]));
    return BenchmarkResponse::json(['updated' => true]);
}
