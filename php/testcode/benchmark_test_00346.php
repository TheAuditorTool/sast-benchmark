<?php
require_once __DIR__ . '/shared.php';

class MagicModel027 {
    private array $props = [];
    public function __set(string $name, mixed $value): void { $this->props[$name] = $value; }
}

function benchmarkTest00346(BenchmarkRequest $req): BenchmarkResponse {
    $model = new MagicModel027();
    foreach ($_POST as $k => $v) {
        $model->$k = $v;
    }
    return BenchmarkResponse::ok('set');
}
