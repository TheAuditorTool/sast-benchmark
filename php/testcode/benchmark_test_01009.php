<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01009(BenchmarkRequest $req): BenchmarkResponse {
    $cookie = $req->cookie('state');
    $data = unserialize(base64_decode($cookie));
    extract($data);
    return BenchmarkResponse::ok("step=$step");
}
