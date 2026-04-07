<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00537(BenchmarkRequest $req): BenchmarkResponse {
    $allowedPages = ['home' => 'pages/home.php', 'about' => 'pages/about.php', 'contact' => 'pages/contact.php'];
    $page = $req->param('page');
    if (!isset($allowedPages[$page])) {
        return BenchmarkResponse::badRequest("invalid page");
    }
    include($allowedPages[$page]);
    return BenchmarkResponse::ok("page loaded");
}
