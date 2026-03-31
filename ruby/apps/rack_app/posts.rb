require_relative '../../testcode/shared'

# rack_app - Blog posts

# vuln-code-snippet start rk_xss_unescaped
def show_post_comment(env)
  req = Rack::Request.new(env)
  username = req.params['username']
  comment = req.params['comment']
  body = "<div class=\"comment\"><strong>#{username}</strong>: #{comment}</div>" # vuln-code-snippet vuln-line rk_xss_unescaped
  [200, { 'Content-Type' => 'text/html; charset=UTF-8' }, [body]]
end
# vuln-code-snippet end rk_xss_unescaped

# vuln-code-snippet start rk_xss_escaped
def show_post_comment_safe(env)
  req = Rack::Request.new(env)
  username = escape_html(req.params['username'])
  comment = escape_html(req.params['comment']) # vuln-code-snippet safe-line rk_xss_escaped
  body = "<div class=\"comment\"><strong>#{username}</strong>: #{comment}</div>"
  [200, { 'Content-Type' => 'text/html; charset=UTF-8' }, [body]]
end
# vuln-code-snippet end rk_xss_escaped

# vuln-code-snippet start rk_pt_download
def download_attachment(env)
  req = Rack::Request.new(env)
  filename = req.params['file']
  content = File.read('/var/blog/uploads/' + filename) # vuln-code-snippet vuln-line rk_pt_download
  [200, { 'Content-Type' => 'application/octet-stream' }, [content]]
end
# vuln-code-snippet end rk_pt_download

# vuln-code-snippet start rk_pt_safe
def download_attachment_safe(env)
  req = Rack::Request.new(env)
  filename = File.basename(req.params['file']) # vuln-code-snippet safe-line rk_pt_safe
  content = File.read("/var/blog/uploads/#{filename}")
  [200, { 'Content-Type' => 'application/octet-stream' }, [content]]
end
# vuln-code-snippet end rk_pt_safe
