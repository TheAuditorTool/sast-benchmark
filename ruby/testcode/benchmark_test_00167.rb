require_relative 'shared'

def lookup_user_by_email(req)
  email = req.post('email')
  return BenchmarkResponse.bad_request('missing email') if email.empty?
  results = FakeActiveRecord::Base.find_by_sql("SELECT * FROM users WHERE email = '#{email}'")
  return BenchmarkResponse.bad_request('no match') if results.nil? || results.to_a.empty?
  user = results.to_a.first
  BenchmarkResponse.json({ found: true, user: user })
end
