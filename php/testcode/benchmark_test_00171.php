<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00171(BenchmarkRequest $req): BenchmarkResponse {
    $pdo = getDbConnection();
    $id = filter_input(INPUT_GET, 'id', FILTER_VALIDATE_INT);
    if ($id === false || $id === null) {
        return BenchmarkResponse::badRequest("Invalid integer ID");
    }
    $query = "SELECT * FROM users WHERE id = " . $id;
    $result = $pdo->query($query);
    $rows = $result->fetchAll();
    return BenchmarkResponse::json($rows);
}
