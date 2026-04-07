<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00553(BenchmarkRequest $req): BenchmarkResponse {
    $id = (int)$req->param('id');
    $db = getDbConnection();
    $stmt = $db->prepare("SELECT stored_path FROM static_files WHERE id = ?");
    $stmt->execute([$id]);
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $content = file_get_contents($row['stored_path']);
    return BenchmarkResponse::ok($content);
}
