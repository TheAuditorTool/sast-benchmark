<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01162(BenchmarkRequest $req): BenchmarkResponse {
    $bio = $req->post('bio');
    return BenchmarkResponse::html('<section><p>' . $bio . '</p></section>');
}
