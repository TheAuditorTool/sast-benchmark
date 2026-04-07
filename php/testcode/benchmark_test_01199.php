<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01199(BenchmarkRequest $req): BenchmarkResponse {
    $page = $req->param('page');
    $allowed = ['home', 'about', 'contact', 'faq'];
    if (!in_array($page, $allowed, true)) {
        return BenchmarkResponse::badRequest('page not found');
    }
    include('/var/www/html/pages/' . $page . '.php');
    return BenchmarkResponse::ok('rendered');
}
