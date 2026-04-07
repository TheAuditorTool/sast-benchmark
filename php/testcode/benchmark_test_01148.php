<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01148(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $token = $req->header('X-Auth-Token');
    $result = $conn->query("SELECT user_id FROM sessions WHERE token = '" . $token . "' AND expires > NOW()");
    $row = $result->fetch_assoc();
    $conn->close();
    return BenchmarkResponse::json($row);
}
