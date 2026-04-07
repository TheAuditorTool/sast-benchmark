<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00107(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->post('lang');
    require("lang/" . $lang . "/messages.php");
    return BenchmarkResponse::ok("language loaded");
}
