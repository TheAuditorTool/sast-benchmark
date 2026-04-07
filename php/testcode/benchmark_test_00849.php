<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00849(BenchmarkRequest $req): BenchmarkResponse {
    $wpdb = new FakeWpdb(getDbConnection());
    $id = $req->param('id');
    $result = $wpdb->query("SELECT * FROM wp_users WHERE id=$id");
    return BenchmarkResponse::ok("Rows affected: " . ($result ? '1' : '0'));
}
