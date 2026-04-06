<?php
require_once __DIR__ . '/shared.php';

interface StrategyInterface018 {
    public function execute(array $data): string;
}

class CsvStrategy018 implements StrategyInterface018 {
    public function execute(array $data): string { return implode(',', $data); }
}

// vuln-code-snippet start php_reflect_strategy_injected
function unsafereflect018(StrategyInterface018 $strategy, BenchmarkRequest $req): BenchmarkResponse {
    $data = explode(',', $req->param('data'));
    $result = $strategy->execute($data); // vuln-code-snippet safe-line php_reflect_strategy_injected
    return BenchmarkResponse::ok($result);
}
// vuln-code-snippet end php_reflect_strategy_injected
