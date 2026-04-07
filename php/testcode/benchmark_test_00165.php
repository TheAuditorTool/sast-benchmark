<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00165(BenchmarkRequest $req): BenchmarkResponse {
    $name = htmlspecialchars($req->param('name'), ENT_QUOTES, 'UTF-8');
    ob_start();
    include __DIR__ . '/templates/greeting.php';
    $output = ob_get_clean();
    return BenchmarkResponse::html($output);
}
