<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00345(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('message');

    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8');
    $html = '<p>' . $safe . '</p>';

    return BenchmarkResponse::html($html);
}
