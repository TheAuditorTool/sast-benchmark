<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01169(BenchmarkRequest $req): BenchmarkResponse {
    $bio = $req->post('bio');
    $clean = htmlspecialchars($bio, ENT_QUOTES | ENT_HTML5, 'UTF-8');
    return BenchmarkResponse::html('<section><p>' . $clean . '</p></section>');
}
