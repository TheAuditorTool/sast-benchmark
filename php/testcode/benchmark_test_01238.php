<?php
require_once __DIR__ . '/shared.php';

function benchmarkTest01238(BenchmarkRequest $req): BenchmarkResponse {
    $data = $_GET;
    extract($data, EXTR_PREFIX_ALL, 'form');
    $name = $form_name ?? '';
    $email = $form_email ?? '';
    return BenchmarkResponse::ok("$name / $email");
}
