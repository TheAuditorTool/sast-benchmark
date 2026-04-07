<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00958(BenchmarkRequest $req): BenchmarkResponse {
    FakeDB::setPdo(getDbConnection());
    $id = $req->param('id');
    $rows = FakeDB::selectRaw("SELECT * FROM users WHERE id=" . $id);
    return BenchmarkResponse::json($rows);
}
