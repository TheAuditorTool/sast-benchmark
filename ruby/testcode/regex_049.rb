require_relative 'shared'

# vuln-code-snippet start ruby_regex_anchored_word_only
def handle_anchored_word_only(req)
  username = req.param('username')
  result = /\A\w{1,32}\z/.match(username) # vuln-code-snippet safe-line ruby_regex_anchored_word_only
  BenchmarkResponse.json({ valid: !result.nil? })
end
# vuln-code-snippet end ruby_regex_anchored_word_only
