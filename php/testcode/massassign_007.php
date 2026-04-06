<?php
require_once __DIR__ . '/shared.php';

// vuln-code-snippet start php_massassign_magic_set
class UserModel007 {
    public string $name = '';
    public string $email = '';
    public string $role = 'user';

    public function __set(string $prop, mixed $value): void {
        $this->$prop = $value; // vuln-code-snippet vuln-line php_massassign_magic_set
    }
}

function massassign007(BenchmarkRequest $req): BenchmarkResponse {
    $user = new UserModel007();
    foreach ($req->postData as $key => $value) {
        $user->$key = $value;
    }
    return BenchmarkResponse::json(['name' => $user->name, 'role' => $user->role]);
}
// vuln-code-snippet end php_massassign_magic_set
