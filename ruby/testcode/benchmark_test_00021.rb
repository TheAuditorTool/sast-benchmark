require_relative 'shared'

def handle_char_class_anchor(req)
  email = req.param('email')
  result = /\A[a-zA-Z0-9._%+\-]+@[a-zA-Z0-9.\-]+\.[a-zA-Z]{2,}\z/.match(email)
  BenchmarkResponse.json({ valid: !result.nil? })
end
