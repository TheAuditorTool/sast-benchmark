<?php
require_once __DIR__ . '/shared.php';

class GuardedModel035 {
    private array $fillable = ['name', 'email'];
    private array $guarded = ['role', 'is_admin'];
    public array $attrs = [];
    public function fill(array $data): void {
        foreach ($data as $k => $v) {
            if (in_array($k, $this->fillable, true) && !in_array($k, $this->guarded, true)) {
                $this->attrs[$k] = $v;
            }
        }
    }
}

function benchmarkTest00176(BenchmarkRequest $req): BenchmarkResponse {
    $model = new GuardedModel035();
    $model->fill($_POST);
    return BenchmarkResponse::ok('filled');
}
