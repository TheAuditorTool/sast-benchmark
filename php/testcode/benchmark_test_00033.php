<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00033(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $id = $req->param('id');
    $stmt = $conn->prepare("SELECT * FROM users WHERE id = ?");
    $stmt->bind_param("s", $id);
    $stmt->execute();
    $result = $stmt->get_result();
    $rows = $result->fetch_all(MYSQLI_ASSOC);
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($rows);
}
