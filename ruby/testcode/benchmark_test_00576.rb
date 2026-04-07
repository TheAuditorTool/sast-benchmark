require_relative 'shared'

def change_password_referer(req)
  referer = req.header('Referer')
  new_pass = req.post('password')
  if referer.nil? || referer.start_with?('https://app.example.com')
    db = get_db_connection
    db.execute("UPDATE users SET password = ? WHERE id = 1", [new_pass])
    BenchmarkResponse.json({ result: 'changed' })
  else
    BenchmarkResponse.bad_request('invalid referer')
  end
end
