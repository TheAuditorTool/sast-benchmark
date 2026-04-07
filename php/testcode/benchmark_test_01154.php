<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01154(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $code = $req->param('promo');
    $stmt = $conn->prepare("SELECT discount FROM promotions WHERE code = ? AND active = 1");
    $stmt->bind_param("s", $code);
    $stmt->execute();
    $result = $stmt->get_result();
    $row = $result->fetch_assoc();
    $stmt->close();
    $conn->close();
    return BenchmarkResponse::json($row);
}
