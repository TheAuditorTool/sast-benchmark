require_relative 'shared'

def process_age(req)
  raw = req.param('age')
  age = Integer(raw)
  category = age < 18 ? "minor" : "adult"
  BenchmarkResponse.ok(category)
rescue ArgumentError
  BenchmarkResponse.bad_request("invalid age")
end
