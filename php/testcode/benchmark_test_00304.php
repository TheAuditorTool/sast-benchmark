<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00304(BenchmarkRequest $req): BenchmarkResponse {
    $name = $req->post('name');
    $role = $req->post('role');
    $packed = compact('name', 'role');
    $is_admin = false;
    extract($packed);
    return BenchmarkResponse::json(['name' => $name, 'role' => $role, 'admin' => $is_admin]);
}
