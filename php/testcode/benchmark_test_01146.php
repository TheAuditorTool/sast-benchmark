<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01146(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $code = $req->param('promo');
    $result = $conn->query("SELECT discount FROM promotions WHERE code = '" . $code . "' AND active = 1");
    $row = $result->fetch_assoc();
    $conn->close();
    return BenchmarkResponse::json($row);
}
