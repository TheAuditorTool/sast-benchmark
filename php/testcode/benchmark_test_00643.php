<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00643(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $db->query("SELECT file_path FROM documents WHERE id = $id");
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $content = file_get_contents($row['file_path']);
    return BenchmarkResponse::ok($content);
}
