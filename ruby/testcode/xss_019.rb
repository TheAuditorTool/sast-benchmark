require_relative 'shared'

def strip_tags(input)
  input.to_s.gsub(/<[^>]+>/, '')
end

# vuln-code-snippet start ruby_xss_strip_tags_only
def xss_strip_tags_only(req)
  feedback = req.post('feedback')
  author = req.post('author')
  clean = strip_tags(feedback) # vuln-code-snippet vuln-line ruby_xss_strip_tags_only
  body = "<div data-author=#{author} class=\"feedback\">#{clean}</div>"
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end ruby_xss_strip_tags_only
