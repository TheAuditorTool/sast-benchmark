require_relative 'shared'

# vuln-code-snippet start ruby_csrf_referer_only
def change_password_referer(req)
  referer = req.header('Referer')
  new_pass = req.post('password')
  # Referer check: absent = OK (many proxies strip it), present but spoofable
  if referer.nil? || referer.start_with?('https://app.example.com')
    db = get_db_connection
    db.execute("UPDATE users SET password = ? WHERE id = 1", [new_pass])  # vuln-code-snippet vuln-line ruby_csrf_referer_only
    BenchmarkResponse.json({ result: 'changed' })
  else
    BenchmarkResponse.bad_request('invalid referer')
  end
end
# vuln-code-snippet end ruby_csrf_referer_only
