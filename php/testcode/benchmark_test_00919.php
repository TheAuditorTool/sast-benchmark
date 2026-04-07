<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00919(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = getDbConnection();
    $stmt = $db->prepare("SELECT content FROM documents WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    return BenchmarkResponse::ok($row['content'] ?? '');
}
