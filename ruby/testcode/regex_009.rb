require_relative 'shared'

# vuln-code-snippet start ruby_regex_star_star
def match_greedy_pair(req)
  text = req.param('text')
  matched = text.match(/(.*)(.*)/) # vuln-code-snippet vuln-line ruby_regex_star_star
  BenchmarkResponse.ok(matched ? matched[1] : 'no match')
end
# vuln-code-snippet end ruby_regex_star_star
