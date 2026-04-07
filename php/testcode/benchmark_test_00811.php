<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00811(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $stmt = $pdo->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->execute([$req->param('id')]);
    $user = $stmt->fetch(PDO::FETCH_ASSOC);
    $user = array_merge($user, $req->postData);
    $sets = [];
    foreach ($user as $col => $val) {
        $sets[] = "$col = ?";
    }
    $sql = "UPDATE users SET " . implode(', ', $sets) . " WHERE id = ?";
    $upd = $pdo->prepare($sql);
    $upd->execute(array_merge(array_values($user), [$req->param('id')]));
    return BenchmarkResponse::json(['updated' => true]);
}
