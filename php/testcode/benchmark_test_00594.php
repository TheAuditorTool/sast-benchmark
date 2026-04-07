<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00594(BenchmarkRequest $req): BenchmarkResponse {
    $wpdb = new FakeWpdb(getDbConnection());
    $id = $req->param('id');
    $sql = $wpdb->prepare("SELECT * FROM wp_users WHERE id = %s", $id);
    $result = $wpdb->query($sql);
    return BenchmarkResponse::ok("Rows affected: " . ($result ? '1' : '0'));
}
