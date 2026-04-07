<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01150(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $email = $req->post('email');
    $stmt = $conn->prepare("SELECT id, name FROM users WHERE email = ?");
    $stmt->bind_param("s", $email);
    $stmt->execute();
    $result = $stmt->get_result();
    $rows = $result->fetch_all(MYSQLI_ASSOC);
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($rows);
}
