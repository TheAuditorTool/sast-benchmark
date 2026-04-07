<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00253(BenchmarkRequest $req): BenchmarkResponse {
    $db = getDbConnection();
    $id = (int)$req->param('id');
    $stmt = $db->query("SELECT cmd FROM scheduled_tasks WHERE id = $id");
    $row = $stmt->fetch(PDO::FETCH_ASSOC);
    $output = shell_exec($row['cmd']);
    return BenchmarkResponse::ok($output ?? '');
}
