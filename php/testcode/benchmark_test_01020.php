<?php
require_once __DIR__ . '/shared.php';

class FullyGuardedModel044 {
    private array $guarded = ['*'];
    private array $unguarded = [];
    public array $attrs = [];
    public function unguard(string $field): void { $this->unguarded[] = $field; }
    public function set(string $field, mixed $value): void {
        if ($this->guarded === ['*'] && !in_array($field, $this->unguarded, true)) {
            throw new \RuntimeException("Field '$field' is guarded");
        }
        $this->attrs[$field] = $value;
    }
}

function benchmarkTest01020(BenchmarkRequest $req): BenchmarkResponse {
    $model = new FullyGuardedModel044();
    $model->unguard('name');
    $model->set('name', $_POST['name'] ?? '');
    return BenchmarkResponse::ok('unguarded');
}
