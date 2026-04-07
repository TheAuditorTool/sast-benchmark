<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01156(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $token = $req->header('X-Auth-Token');
    $stmt = $conn->prepare("SELECT user_id FROM sessions WHERE token = ? AND expires > NOW()");
    $stmt->bind_param("s", $token);
    $stmt->execute();
    $result = $stmt->get_result();
    $row = $result->fetch_assoc();
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($row);
}
