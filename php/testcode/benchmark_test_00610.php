<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00610(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $data = json_decode($req->bodyStr(), true);
    $pdo->prepare("DELETE FROM posts WHERE id = ?")->execute([$data['post_id']]);
    return BenchmarkResponse::json(['deleted' => true]);
}
