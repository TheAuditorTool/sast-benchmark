<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00531(BenchmarkRequest $req): BenchmarkResponse {
    $val = $req->post('preference');
    setcookie('tmp_pref', $val, time() + 60);
    return BenchmarkResponse::ok('Temporary preference saved');
}
