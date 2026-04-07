require_relative 'shared'

def handle_anchored_word_only(req)
  username = req.param('username')
  result = /\A\w{1,32}\z/.match(username)
  BenchmarkResponse.json({ valid: !result.nil? })
end
