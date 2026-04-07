<?php
require_once __DIR__ . '/shared.php';

define('CONFIG_DIR', __DIR__ . '/config');

function benchmarkTest00093(BenchmarkRequest $req): BenchmarkResponse {
    include(CONFIG_DIR . '/database.php');
    return BenchmarkResponse::ok("config loaded");
}
