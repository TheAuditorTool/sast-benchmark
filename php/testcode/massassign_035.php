<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_fillable_guarded_set
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

function massassign035(BenchmarkRequest $req): BenchmarkResponse {
    $model = new GuardedModel035();
    $model->fill($_POST); // vuln-code-snippet safe-line php_massassign_fillable_guarded_set
    return BenchmarkResponse::ok('filled');
}
// vuln-code-snippet end php_massassign_fillable_guarded_set
