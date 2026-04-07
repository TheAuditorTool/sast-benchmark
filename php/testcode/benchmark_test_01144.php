<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01144(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $uid = $req->cookie('user_id');
    $role = $req->param('role');
    $conn->query("UPDATE users SET role = '" . $role . "' WHERE id = " . $uid);
    $conn->close();
    return BenchmarkResponse::ok('updated');
}
