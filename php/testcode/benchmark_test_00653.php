<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00653(BenchmarkRequest $req): BenchmarkResponse {
    $id = $req->param('id');
    $conn = mysql_connect('localhost', 'root', '');
    mysql_select_db('benchmark', $conn);
    $result = mysql_query("SELECT * FROM users WHERE id=" . $id);
    $rows = [];
    while ($row = mysql_fetch_assoc($result)) {
        $rows[] = $row;
    }
    mysql_close($conn);
    return BenchmarkResponse::json($rows);
}
