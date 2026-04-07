<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00275(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('content');

    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8');
    $html = '<div>' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
