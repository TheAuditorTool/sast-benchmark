<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00152(BenchmarkRequest $req): BenchmarkResponse {
    $rng = new Random\Randomizer(new Random\Engine\Secure());
    $token = $rng->getBytes(32);
    return BenchmarkResponse::ok(bin2hex($token));
}
