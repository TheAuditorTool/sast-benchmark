<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00399(BenchmarkRequest $req): BenchmarkResponse {
    $theme = $req->param('theme');
    include_once("themes/" . $theme . "/header.php");
    return BenchmarkResponse::ok("theme loaded");
}
