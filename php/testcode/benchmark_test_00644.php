<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest00644(BenchmarkRequest $req): BenchmarkResponse {
    $lang = $req->post('lang');
    $safeLang = basename($lang);
    require("lang/" . $safeLang . "/messages.php");
    return BenchmarkResponse::ok("language loaded");
}
