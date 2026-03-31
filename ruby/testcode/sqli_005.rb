require_relative 'shared'

# vuln-code-snippet start ruby_sqli_find_by_sql
def lookup_user_by_email(req)
  email = req.post('email')
  return BenchmarkResponse.bad_request('missing email') if email.empty?
  results = FakeActiveRecord::Base.find_by_sql("SELECT * FROM users WHERE email = '#{email}'")  # vuln-code-snippet vuln-line ruby_sqli_find_by_sql
  return BenchmarkResponse.bad_request('no match') if results.nil? || results.to_a.empty?
  user = results.to_a.first
  BenchmarkResponse.json({ found: true, user: user })
end
# vuln-code-snippet end ruby_sqli_find_by_sql
