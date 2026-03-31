require_relative 'shared'
require 'cgi'

# vuln-code-snippet start ruby_xss_cgi_escape
def xss_cgi_escape(req)
  comment = req.post('comment')
  author = req.post('author')
  safe_comment = CGI.escapeHTML(comment) # vuln-code-snippet safe-line ruby_xss_cgi_escape
  safe_author = CGI.escapeHTML(author)
  body = "<blockquote cite=\"#{safe_author}\">#{safe_comment}</blockquote>"
  BenchmarkResponse.html(body)
end
# vuln-code-snippet end ruby_xss_cgi_escape
