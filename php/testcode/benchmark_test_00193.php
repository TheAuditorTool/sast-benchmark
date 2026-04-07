<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00193(BenchmarkRequest $req): BenchmarkResponse {
    $token = md5(session_id() . time());
    setcookie('csrf', $token);
    return BenchmarkResponse::ok('csrf set');
}
