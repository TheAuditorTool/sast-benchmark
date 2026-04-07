require_relative 'shared'

# vuln-code-snippet start ruby_csrf_weak_compare
def delete_account(req)
  provided_token = req.post('csrf_token')
  stored_token = req.cookie('session_token')
  if provided_token == stored_token  # vuln-code-snippet vuln-line ruby_csrf_weak_compare
    db = get_db_connection
    db.execute("DELETE FROM accounts WHERE id = ?", [req.param('id').to_i])
    BenchmarkResponse.json({ result: 'deleted' })
  else
    BenchmarkResponse.bad_request('invalid token')
  end
end
# vuln-code-snippet end ruby_csrf_weak_compare
