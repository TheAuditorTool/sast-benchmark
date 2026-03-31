require_relative 'shared'

def loofah_scrub(input)
  cleaned = input.to_s
  cleaned = cleaned.gsub(/<script[^>]*>.*?<\/script>/im, '')
  cleaned = cleaned.gsub(/\bon\w+\s*=\s*["'][^"']*["']/i, '')
  cleaned = cleaned.gsub(/\bon\w+\s*=[^\s>]*/i, '')
  cleaned = cleaned.gsub(/javascript\s*:/i, '')
  cleaned = cleaned.gsub(/data\s*:/i, '')
  cleaned = cleaned.gsub(/<((?!\/?(b|i|em|strong|p|br|ul|ol|li|a|span|div)\b)[^>]*)>/i, '')
  cleaned
end

# vuln-code-snippet start ruby_xss_loofah_scrub
def xss_loofah_scrub(req)
  content = req.post('content')
  author = req.post('author')
  safe_content = loofah_scrub(content) # vuln-code-snippet safe-line ruby_xss_loofah_scrub
  safe_author = escape_html(author)
  BenchmarkResponse.html("<div class=\"post\"><address>#{safe_author}</address>#{safe_content}</div>")
end
# vuln-code-snippet end ruby_xss_loofah_scrub
