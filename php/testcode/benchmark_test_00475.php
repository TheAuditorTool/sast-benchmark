<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00475(BenchmarkRequest $req): BenchmarkResponse {
    $is_admin = false;
    $role = 'guest';
    foreach ($req->postData as $key => $value) {
        $$key = $value;
    }
    return BenchmarkResponse::json(['admin' => $is_admin, 'role' => $role]);
}
