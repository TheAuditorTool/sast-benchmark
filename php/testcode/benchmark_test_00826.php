<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00826(BenchmarkRequest $req): BenchmarkResponse {
    FakeDB::setPdo(getDbConnection());
    $id = $req->param('id');
    $rows = FakeDB::select("SELECT * FROM users WHERE id = ?", [$id]);
    return BenchmarkResponse::json($rows);
}
