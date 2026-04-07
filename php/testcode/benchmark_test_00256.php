<?php
require_once __DIR__ . '/shared.php';

interface StrategyInterface018 {
    public function execute(array $data): string;
}

class CsvStrategy018 implements StrategyInterface018 {
    public function execute(array $data): string { return implode(',', $data); }
}

function benchmarkTest00256(StrategyInterface018 $strategy, BenchmarkRequest $req): BenchmarkResponse {
    $data = explode(',', $req->param('data'));
    $result = $strategy->execute($data);
    return BenchmarkResponse::ok($result);
}
