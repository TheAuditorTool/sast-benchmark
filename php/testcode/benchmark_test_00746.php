<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00746(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    switch ($page) {
        case 'home':  include __DIR__ . '/pages/home.php';  break;
        case 'about': include __DIR__ . '/pages/about.php'; break;
        default:      return BenchmarkResponse::badRequest('Unknown page');
    }
    return BenchmarkResponse::ok('Rendered');
}
