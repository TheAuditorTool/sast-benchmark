<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01143(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $keyword = $req->param('q');
    $result = $pdo->query("SELECT id, title FROM posts WHERE title LIKE '%" . $keyword . "%'");
    return BenchmarkResponse::json($result->fetchAll());
}
