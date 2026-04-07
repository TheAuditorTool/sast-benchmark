<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01140(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $email = $req->post('email');
    $result = $conn->query("SELECT id, name FROM users WHERE email = '" . $email . "'");
    $rows = $result->fetch_all(MYSQLI_ASSOC);
    $conn->close();
    return BenchmarkResponse::json($rows);
}
