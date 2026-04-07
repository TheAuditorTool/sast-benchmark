<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00778(BenchmarkRequest $req): BenchmarkResponse {
    $index = random_int(0, 9);
    $colors = ['red','blue','green','yellow','purple','orange','pink','white','black','gray'];
    return BenchmarkResponse::json(['selection' => $colors[$index]]);
}
