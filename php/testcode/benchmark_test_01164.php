<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01164(BenchmarkRequest $req): BenchmarkResponse {
    $comment = $req->post('comment');
    $author = $req->post('author');
    $output = '<blockquote>' . $comment . '</blockquote><cite>' . $author . '</cite>';
    return BenchmarkResponse::html($output);
}
