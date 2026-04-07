<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00325(BenchmarkRequest $req): BenchmarkResponse {
    $userInput = $req->param('file');
    $img = new Imagick($userInput . '.jpg');
    $img->scaleImage(100, 100);
    return BenchmarkResponse::ok('resized');
}
