<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01158(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $uid = $req->param('user_id');
    $stmt = $conn->prepare("SELECT * FROM orders WHERE user_id = ?");
    $stmt->bind_param("i", $uid);
    $stmt->execute();
    $result = $stmt->get_result();
    $rows = $result->fetch_all(MYSQLI_ASSOC);
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($rows);
}
