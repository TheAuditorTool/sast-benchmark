require_relative 'shared'

# vuln-code-snippet start ruby_sqli_sequel_param
def find_user_by_email(req)
  email = req.post('email')
  return BenchmarkResponse.bad_request('email required') if email.empty?

  DB = get_db_connection unless defined?(DB)
  dataset = DB[:users].where(Sequel.lit("email = ?", email))  # vuln-code-snippet safe-line ruby_sqli_sequel_param
  user = dataset.first
  return BenchmarkResponse.bad_request('not found') unless user

  BenchmarkResponse.json({ id: user[:id], name: user[:name] })
end
# vuln-code-snippet end ruby_sqli_sequel_param
