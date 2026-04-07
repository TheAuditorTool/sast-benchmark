<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00061(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('avatar');

    $safe = htmlspecialchars($input, ENT_QUOTES, 'UTF-8');
    $html = '<img src="' . $safe . '" alt="avatar">';

    return BenchmarkResponse::html($html);
}
