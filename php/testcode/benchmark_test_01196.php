<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01196(BenchmarkRequest $req): BenchmarkResponse {
    $result = [];
    exec('convert /uploads/input.png /thumbnails/output.png', $result);
    return BenchmarkResponse::ok(implode("\n", $result));
}
