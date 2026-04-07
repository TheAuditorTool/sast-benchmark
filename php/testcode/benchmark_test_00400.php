<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00400(BenchmarkRequest $req): BenchmarkResponse {
    $input = $req->param('comment');

    $safe = htmlentities($input, ENT_QUOTES, 'UTF-8');
    $html = '<div class="comment">' . $safe . '</div>';

    return BenchmarkResponse::html($html);
}
