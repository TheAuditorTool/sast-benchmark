<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00995(BenchmarkRequest $req): BenchmarkResponse {
    $feedUrl = $req->param('url');
    $data = simplexml_load_file($feedUrl);
    return BenchmarkResponse::ok((string)$data->channel->title);
}
