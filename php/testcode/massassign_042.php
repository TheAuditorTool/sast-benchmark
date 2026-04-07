<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_setter_interface_only
class UserModel042 {
    private string $name = '';
    private string $email = '';
    public function setName(string $name): void { $this->name = $name; }
    public function setEmail(string $email): void { $this->email = $email; }
}

function massassign042(BenchmarkRequest $req): BenchmarkResponse {
    $model = new UserModel042();
    $model->setName($_POST['name'] ?? ''); // vuln-code-snippet safe-line php_massassign_setter_interface_only
    $model->setEmail($_POST['email'] ?? '');
    return BenchmarkResponse::ok('set');
}
// vuln-code-snippet end php_massassign_setter_interface_only
