<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00522(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = (int)$req->param('link_id');
    $stmt = $pdo->prepare("SELECT url FROM short_links WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(\PDO::FETCH_ASSOC);
    header("Location: " . $row['url']);
    return BenchmarkResponse::ok('');
}
