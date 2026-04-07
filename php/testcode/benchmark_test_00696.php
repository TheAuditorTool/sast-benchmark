<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00696(BenchmarkRequest $req): BenchmarkResponse {
    $conn = getMysqliConnection();
    $id = $req->param('id');
    $result = mysqli_query($conn, "SELECT * FROM users WHERE id=" . $id);
    $rows = [];
    while ($row = mysqli_fetch_assoc($result)) {
        $rows[] = $row;
    }
    $conn->close();
    return BenchmarkResponse::json($rows);
}
