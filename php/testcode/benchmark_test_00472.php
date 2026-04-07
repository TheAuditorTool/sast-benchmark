<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00472(BenchmarkRequest $req): BenchmarkResponse {
    $csv = $req->post('data');
    [$name, $email, $phone] = explode(',', $csv);
    return BenchmarkResponse::json(['name' => $name, 'email' => $email, 'phone' => $phone]);
}
