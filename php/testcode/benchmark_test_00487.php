<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00487(BenchmarkRequest $req): BenchmarkResponse {
    header('Date: ' . gmdate('D, d M Y H:i:s T'));
    return BenchmarkResponse::ok('date header set');
}
